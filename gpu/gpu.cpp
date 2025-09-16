#include "gpu.h"
#include "utils.h"
#include <fstream>
#include <sstream>
#include <iostream>
#include <algorithm>
#include <unordered_map>
#include <string_view>
#include <regex>
#include <cstdlib>
#include <cstdio>
#include <dirent.h>

#ifdef __linux__
#include <glob.h>
#endif

namespace kfetch {

// Static helper to run commands and capture output
std::string GPUInfo::runCommand(const char* cmd) const {
    FILE* pipe = popen(cmd, "r");
    if (!pipe) return {};
    
    char buffer[1024];
    std::string result;
    while (fgets(buffer, sizeof(buffer), pipe)) {
        result += buffer;
    }
    pclose(pipe);
    
    return trim(result);
}

GPUInfo::GPUInfo() {
#ifdef __linux__
    detectLinux();
#elif defined(__FreeBSD__) || defined(__DragonFly__)
    detectFreeBSD();
#elif defined(__OpenBSD__)
    detectOpenBSD();
#elif defined(__NetBSD__)
    detectNetBSD();
#elif defined(__APPLE__)
    detectMacOS();
#endif

    // If no GPUs detected, add unknown entry
    if (gpus.empty()) {
        GPUDevice unknown;
        unknown.name = "Unknown GPU";
        unknown.vendor = "Unknown";
        unknown.is_primary = true;
        gpus.push_back(unknown);
    }
}

void GPUInfo::detectLinux() {
    // Method 1: Try lspci first (most reliable)
    if (system("command -v lspci >/dev/null 2>&1") == 0) {
        std::string lspci_output = runCommand(
            "lspci -nn | grep -E '(VGA|3D|Display).*controller'");
        
        if (!lspci_output.empty()) {
            std::istringstream stream(lspci_output);
            std::string line;
            while (std::getline(stream, line)) {
                GPUDevice gpu;
                gpu.name = extractGPUFromLspci(line);
                
                // Extract PCI ID
                std::regex pci_regex(R"(\[([0-9a-fA-F]{4}:[0-9a-fA-F]{4})\])");
                std::smatch match;
                if (std::regex_search(line, match, pci_regex)) {
                    gpu.pci_id = match[1].str();
                }
                
                // Parse vendor-specific info
                if (gpu.name.find("NVIDIA") != std::string::npos || 
                    gpu.name.find("GeForce") != std::string::npos ||
                    gpu.name.find("Quadro") != std::string::npos) {
                    gpu.vendor = "NVIDIA";
                    parseNvidiaInfo(gpu);
                } else if (gpu.name.find("AMD") != std::string::npos ||
                          gpu.name.find("Radeon") != std::string::npos ||
                          gpu.name.find("ATI") != std::string::npos) {
                    gpu.vendor = "AMD";
                    parseAMDInfo(gpu);
                } else if (gpu.name.find("Intel") != std::string::npos) {
                    gpu.vendor = "Intel";
                    parseIntelInfo(gpu);
                }
                
                gpu.name = cleanGPUName(gpu.name);
                gpus.push_back(gpu);
            }
        }
    }
    
    // Method 2: NVIDIA-specific fallback
    if (gpus.empty() && system("command -v nvidia-smi >/dev/null 2>&1") == 0) {
        std::string nvidia_output = runCommand(
            "nvidia-smi --query-gpu=name,memory.total,driver_version --format=csv,noheader,nounits 2>/dev/null");
        
        if (!nvidia_output.empty()) {
            std::istringstream stream(nvidia_output);
            std::string line;
            while (std::getline(stream, line)) {
                GPUDevice gpu;
                std::istringstream linestream(line);
                std::string item;
                
                if (std::getline(linestream, item, ',')) gpu.name = trim(item);
                if (std::getline(linestream, item, ',')) gpu.memory = trim(item) + " MB";
                if (std::getline(linestream, item, ',')) gpu.driver = trim(item);
                
                gpu.vendor = "NVIDIA";
                gpus.push_back(gpu);
            }
        }
    }
    
    // Method 3: Try /sys/class/drm (Linux only)
    if (gpus.empty()) {
#ifdef __linux__
        glob_t glob_result;
        if (glob("/sys/class/drm/card*/device/vendor", GLOB_TILDE, nullptr, &glob_result) == 0) {
            for (size_t i = 0; i < glob_result.gl_pathc; ++i) {
                std::ifstream vendor_file(glob_result.gl_pathv[i]);
                std::string vendor_id;
                if (std::getline(vendor_file, vendor_id)) {
                    std::string device_path = std::string(glob_result.gl_pathv[i]);
                    device_path.replace(device_path.find("vendor"), 6, "device");
                    
                    std::ifstream device_file(device_path);
                    std::string device_id;
                    if (std::getline(device_file, device_id)) {
                        GPUDevice gpu;
                        gpu.pci_id = vendor_id.substr(2) + ":" + device_id.substr(2);
                        gpu.name = "PCI Device " + gpu.pci_id;
                        gpus.push_back(gpu);
                    }
                }
            }
            globfree(&glob_result);
        }
#else
        // Alternative /sys parsing for non-Linux systems that might have it
        DIR* drm_dir = opendir("/sys/class/drm");
        if (drm_dir) {
            struct dirent* entry;
            while ((entry = readdir(drm_dir)) != nullptr) {
                std::string dirname = entry->d_name;
                if (dirname.find("card") == 0 && dirname.find("card") != std::string::npos) {
                    std::string vendor_path = "/sys/class/drm/" + dirname + "/device/vendor";
                    std::ifstream vendor_file(vendor_path);
                    std::string vendor_id;
                    if (std::getline(vendor_file, vendor_id)) {
                        GPUDevice gpu;
                        gpu.name = "DRM Device " + dirname;
                        gpus.push_back(gpu);
                    }
                }
            }
            closedir(drm_dir);
        }
#endif
    }
    
    // Method 4: OpenGL fallback
    if (gpus.empty() && system("command -v glxinfo >/dev/null 2>&1") == 0) {
        std::string gl_renderer = runCommand(
            "glxinfo 2>/dev/null | grep 'OpenGL renderer string' | cut -d: -f2");
        if (!gl_renderer.empty()) {
            GPUDevice gpu;
            gpu.name = cleanGPUName(trim(gl_renderer));
            gpu.driver = "OpenGL";
            gpus.push_back(gpu);
        }
    }
}

void GPUInfo::detectFreeBSD() {
    // Method 1: NVIDIA first (if available)
    if (system("command -v nvidia-smi >/dev/null 2>&1") == 0) {
        std::string nvidia_output = runCommand(
            "nvidia-smi --query-gpu=name,memory.total,driver_version --format=csv,noheader,nounits 2>/dev/null");
        
        if (!nvidia_output.empty()) {
            std::istringstream stream(nvidia_output);
            std::string line;
            while (std::getline(stream, line)) {
                GPUDevice gpu;
                std::istringstream linestream(line);
                std::string item;
                
                if (std::getline(linestream, item, ',')) gpu.name = trim(item);
                if (std::getline(linestream, item, ',')) gpu.memory = trim(item) + " MB";
                if (std::getline(linestream, item, ',')) gpu.driver = trim(item);
                
                gpu.vendor = "NVIDIA";
                gpus.push_back(gpu);
            }
        }
    }
    
    // Method 2: pciconf
    if (gpus.empty()) {
        std::string pciconf_output = runCommand(
            "pciconf -lv | grep -i -A 5 -B 1 -E '(vga|display|nvidia|amd|radeon|intel)'");
        
        if (!pciconf_output.empty()) {
            std::string gpu_name = extractGPUFromPciconf(pciconf_output);
            if (!gpu_name.empty()) {
                GPUDevice gpu;
                gpu.name = cleanGPUName(gpu_name);
                gpus.push_back(gpu);
            }
        }
    }
    
    // Method 3: dmesg fallback
    if (gpus.empty()) {
        std::string dmesg_gpu = extractGPUFromDmesg(
            "dmesg | grep -i -E '(nvidia|amd|radeon|intel).*graphics|vga.*pci' | head -1");
        if (!dmesg_gpu.empty()) {
            GPUDevice gpu;
            gpu.name = cleanGPUName(dmesg_gpu);
            gpus.push_back(gpu);
        }
    }
    
    // Method 4: OpenGL fallback
    if (gpus.empty() && system("command -v glxinfo >/dev/null 2>&1") == 0) {
        std::string gl_renderer = runCommand(
            "glxinfo 2>/dev/null | grep 'OpenGL renderer string' | cut -d: -f2");
        if (!gl_renderer.empty()) {
            GPUDevice gpu;
            gpu.name = cleanGPUName(trim(gl_renderer));
            gpu.driver = "OpenGL";
            gpus.push_back(gpu);
        }
    }
}

void GPUInfo::detectOpenBSD() {
    // Method 1: Look for DRM driver attachments (most reliable)
    std::string drm_output = runCommand(
        "dmesg | grep -E '(inteldrm|amdgpu|radeondrm|nvidia).*at.*pci' | head -3");
    
    if (!drm_output.empty()) {
        std::istringstream stream(drm_output);
        std::string line;
        while (std::getline(stream, line)) {
            GPUDevice gpu;
            gpu.name = cleanGPUName(line);
            
            if (line.find("inteldrm") != std::string::npos) {
                gpu.vendor = "Intel";
                gpu.driver = "inteldrm";
            } else if (line.find("amdgpu") != std::string::npos) {
                gpu.vendor = "AMD";
                gpu.driver = "amdgpu";
            } else if (line.find("radeondrm") != std::string::npos) {
                gpu.vendor = "AMD";
                gpu.driver = "radeondrm";
            } else if (line.find("nvidia") != std::string::npos) {
                gpu.vendor = "NVIDIA";
            }
            
            gpus.push_back(gpu);
        }
    }
    
    // Method 2: VGA controller attachments
    if (gpus.empty()) {
        std::string vga_output = runCommand(
            "dmesg | grep -E 'vga[0-9]+.*at.*pci.*' | head -3");
        
        if (!vga_output.empty()) {
            std::istringstream stream(vga_output);
            std::string line;
            while (std::getline(stream, line)) {
                GPUDevice gpu;
                gpu.name = cleanGPUName(line);
                gpus.push_back(gpu);
            }
        }
    }
    
    // Method 3: Framebuffer devices
    if (gpus.empty()) {
        std::string fb_output = runCommand(
            "dmesg | grep -E '(efifb|vesafb)[0-9]*.*' | head -1");
        if (!fb_output.empty()) {
            GPUDevice gpu;
            gpu.name = cleanGPUName(fb_output);
            gpu.driver = "framebuffer";
            gpus.push_back(gpu);
        }
    }
    
    // Method 4: PCI device listing fallback
    if (gpus.empty()) {
        std::string pci_output = runCommand(
            "dmesg | grep -i -E '(display|graphics|video).*controller.*pci' | head -1");
        if (!pci_output.empty()) {
            GPUDevice gpu;
            gpu.name = cleanGPUName(pci_output);
            gpus.push_back(gpu);
        }
    }
    
    // Method 5: Broad search fallback
    if (gpus.empty()) {
        std::string broad_output = runCommand(
            "dmesg | grep -i -E '(intel|amd|nvidia|ati|radeon).*graphics' | head -1");
        if (!broad_output.empty()) {
            GPUDevice gpu;
            gpu.name = cleanGPUName(broad_output);
            gpus.push_back(gpu);
        }
    }
}

void GPUInfo::detectNetBSD() {
    // Similar to OpenBSD but with NetBSD-specific patterns
    std::string dmesg_output = runCommand(
        "dmesg | grep -i -E '(vga|graphics|nvidia|amd|radeon|intel|drm)' | head -3");
    
    if (!dmesg_output.empty()) {
        std::istringstream stream(dmesg_output);
        std::string line;
        while (std::getline(stream, line)) {
            GPUDevice gpu;
            gpu.name = cleanGPUName(line);
            gpus.push_back(gpu);
        }
    }
}

void GPUInfo::detectMacOS() {
    // Method 1: system_profiler (most detailed)
    std::string profiler_output = runCommand(
        "system_profiler SPDisplaysDataType | grep 'Chipset Model' | cut -d: -f2");
    
    if (!profiler_output.empty()) {
        std::istringstream stream(profiler_output);
        std::string line;
        while (std::getline(stream, line)) {
            GPUDevice gpu;
            gpu.name = cleanGPUName(trim(line));
            gpu.vendor = "Apple/Intel/AMD"; // macOS typically uses these
            gpus.push_back(gpu);
        }
    }
    
    // Method 2: ioreg fallback
    if (gpus.empty()) {
        std::string ioreg_output = runCommand(
            "ioreg -l | grep -i 'model.*gpu\\|graphics' | head -1");
        if (!ioreg_output.empty()) {
            GPUDevice gpu;
            gpu.name = cleanGPUName(ioreg_output);
            gpus.push_back(gpu);
        }
    }
}

std::string GPUInfo::extractGPUFromLspci(const std::string& line) const {
    // Extract GPU name from lspci output
    if (auto colon = line.find(": "); colon != std::string::npos) {
        return trim(line.substr(colon + 2));
    }
    return trim(line);
}

std::string GPUInfo::extractGPUFromPciconf(const std::string& output) const {
    std::string vendor, device;
    std::istringstream stream(output);
    std::string line;
    
    while (std::getline(stream, line)) {
        if (line.find("vendor = '") != std::string::npos) {
            size_t start = line.find("'") + 1;
            size_t end = line.find("'", start);
            if (end != std::string::npos) {
                vendor = line.substr(start, end - start);
            }
        }
        if (line.find("device = '") != std::string::npos) {
            size_t start = line.find("'") + 1;
            size_t end = line.find("'", start);
            if (end != std::string::npos) {
                device = line.substr(start, end - start);
            }
        }
    }
    
    if (!vendor.empty() && !device.empty()) {
        return vendor + " " + device;
    } else if (!device.empty()) {
        return device;
    } else if (!vendor.empty()) {
        return vendor;
    }
    
    return "";
}

std::string GPUInfo::extractGPUFromDmesg(const std::string& pattern) const {
    return runCommand(pattern.c_str());
}

std::string GPUInfo::cleanGPUName(const std::string& raw_name) const {
    if (raw_name.empty()) return raw_name;
    
    std::string cleaned = raw_name;
    
    // Remove common prefixes/suffixes and clean up
    static const std::unordered_map<std::string_view, std::string_view> replacements = {
        {"Advanced Micro Devices, Inc.", "AMD"},
        {"Advanced Micro Devices", "AMD"},
        {"Intel Corporation", "Intel"},
        {"NVIDIA Corporation", "NVIDIA"},
        {"Corporation", ""},
        {"Inc.", ""},
        {" [AMD/ATI]", ""},
        {"vendor = '", ""},
        {"device = '", ""},
        {"VGA compatible controller:", ""},
        {"Display controller:", ""},
        {"3D controller:", ""}
    };
    
    for (auto [from, to] : replacements) {
        size_t pos = 0;
        while ((pos = cleaned.find(from, pos)) != std::string::npos) {
            cleaned.replace(pos, from.size(), std::string(to));
            pos += to.size();
        }
    }
    
    // Remove quotes, brackets, and extra punctuation
    std::erase_if(cleaned, [](char c) { 
        return c == '\'' || c == '\"' || c == '[' || c == ']' || 
               c == '(' || c == ')' || c == ','; 
    });
    
    // Collapse multiple spaces
    std::regex multi_space("\\s+");
    cleaned = std::regex_replace(cleaned, multi_space, " ");
    
    return trim(cleaned);
}

void GPUInfo::parseNvidiaInfo(GPUDevice& gpu) const {
    if (system("command -v nvidia-smi >/dev/null 2>&1") == 0) {
        std::string memory = runCommand(
            "nvidia-smi --query-gpu=memory.total --format=csv,noheader,nounits 2>/dev/null | head -1");
        if (!memory.empty()) {
            gpu.memory = trim(memory) + " MB";
        }
        
        std::string driver = runCommand(
            "nvidia-smi --query-gpu=driver_version --format=csv,noheader 2>/dev/null | head -1");
        if (!driver.empty()) {
            gpu.driver = trim(driver);
        }
    }
}

void GPUInfo::parseAMDInfo(GPUDevice& gpu) const {
#ifdef __linux__
    // Try to get AMD-specific information on Linux
    std::ifstream drm_memory("/sys/class/drm/card0/device/mem_info_vram_total");
    if (drm_memory.is_open()) {
        std::string memory_bytes;
        if (std::getline(drm_memory, memory_bytes)) {
            try {
                long long bytes = std::stoll(memory_bytes);
                long long mb = bytes / (1024 * 1024);
                gpu.memory = std::to_string(mb) + " MB";
            } catch (...) {}
        }
    }
#endif
    // Set driver info
    gpu.driver = "AMD Graphics";
}

void GPUInfo::parseIntelInfo(GPUDevice& gpu) const {
    // Intel integrated graphics info is harder to get
    gpu.driver = "Intel Graphics";
}

std::string GPUInfo::getPrimaryGPU() const {
    if (gpus.empty()) return "Unknown GPU";
    
    // Look for a primary GPU first
    for (const auto& gpu : gpus) {
        if (gpu.is_primary) {
            return gpu.name;
        }
    }
    
    // If no primary, return first GPU
    return gpus[0].name;
}

std::string GPUInfo::getFormatted() const {
    if (gpus.empty()) return "Unknown GPU";
    
    if (gpus.size() == 1) {
        return gpus[0].name;
    }
    
    // Multiple GPUs - show primary or first + count
    std::string result = getPrimaryGPU();
    if (gpus.size() > 1) {
        result += " (+" + std::to_string(gpus.size() - 1) + " more)";
    }
    
    return result;
}

bool GPUInfo::hasNvidiaGPU() const {
    return std::any_of(gpus.begin(), gpus.end(), 
        [](const GPUDevice& gpu) { return gpu.vendor == "NVIDIA"; });
}

bool GPUInfo::hasAMDGPU() const {
    return std::any_of(gpus.begin(), gpus.end(), 
        [](const GPUDevice& gpu) { return gpu.vendor == "AMD"; });
}

bool GPUInfo::hasIntelGPU() const {
    return std::any_of(gpus.begin(), gpus.end(), 
        [](const GPUDevice& gpu) { return gpu.vendor == "Intel"; });
}

void GPUInfo::refresh() {
    gpus.clear();
    formatted_output.clear();
    
    // Re-run detection
    GPUInfo temp;
    *this = std::move(temp);
}

#ifdef DEBUG_GPU
void GPUInfo::debugOutput() const {
    std::cout << "=== GPU Detection Debug ===" << std::endl;
    std::cout << "Found " << gpus.size() << " GPU(s):" << std::endl;
    
    for (size_t i = 0; i < gpus.size(); ++i) {
        const auto& gpu = gpus[i];
        std::cout << "GPU " << i << ":" << std::endl;
        std::cout << "  Name: " << gpu.name << std::endl;
        std::cout << "  Vendor: " << gpu.vendor << std::endl;
        std::cout << "  Driver: " << gpu.driver << std::endl;
        std::cout << "  Memory: " << gpu.memory << std::endl;
        std::cout << "  PCI ID: " << gpu.pci_id << std::endl;
        std::cout << "  Primary: " << (gpu.is_primary ? "Yes" : "No") << std::endl;
        std::cout << std::endl;
    }
    
    // Debug system commands
    std::cout << "=== System Command Debug ===" << std::endl;
    
#ifdef __linux__
    std::cout << "lspci VGA: " << runCommand("lspci | grep VGA") << std::endl;
    std::cout << "nvidia-smi: " << runCommand("nvidia-smi -L 2>/dev/null || echo 'Not available'") << std::endl;
#elif defined(__OpenBSD__)
    std::cout << "dmesg DRM: " << runCommand("dmesg | grep -E 'inteldrm|amdgpu|radeondrm' | head -1") << std::endl;
    std::cout << "dmesg VGA: " << runCommand("dmesg | grep 'vga.*at.*pci' | head -1") << std::endl;
#elif defined(__FreeBSD__)
    std::cout << "pciconf: " << runCommand("pciconf -lv | grep -A 2 vgapci") << std::endl;
#endif
}
#endif

} // namespace kfetch
