#ifndef GPU_H
#define GPU_H

#include <string>
#include <vector>

namespace kfetch {

struct GPUDevice {
    std::string name;
    std::string vendor;
    std::string driver;
    std::string memory;
    std::string pci_id;
    bool is_primary = false;
};

class GPUInfo {
private:
    std::vector<GPUDevice> gpus;
    std::string formatted_output;
    
    // Internal detection methods
    void detectLinux();
    void detectFreeBSD();
    void detectOpenBSD();
    void detectNetBSD();
    void detectMacOS();
    
    // Helper methods
    std::string runCommand(const char* cmd) const;
    std::string extractGPUFromLspci(const std::string& line) const;
    std::string extractGPUFromPciconf(const std::string& output) const;
    std::string extractGPUFromDmesg(const std::string& pattern) const;
    std::string cleanGPUName(const std::string& raw_name) const;
    void parseNvidiaInfo(GPUDevice& gpu) const;
    void parseAMDInfo(GPUDevice& gpu) const;
    void parseIntelInfo(GPUDevice& gpu) const;
    
public:
    GPUInfo();
    
    // Getters
    const std::vector<GPUDevice>& getGPUs() const { return gpus; }
    std::string getPrimaryGPU() const;
    std::string getFormatted() const;
    size_t getGPUCount() const { return gpus.size(); }
    
    // Utility methods
    bool hasNvidiaGPU() const;
    bool hasAMDGPU() const;
    bool hasIntelGPU() const;
    void refresh(); // Re-detect GPUs
    
#ifdef DEBUG_GPU
    void debugOutput() const;
#endif
};

} // namespace kfetch

#endif // GPU_H
