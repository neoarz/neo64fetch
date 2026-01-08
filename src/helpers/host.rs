use std::process::Command;

pub fn get_host_info() -> String {
    let model_id = Command::new("sysctl")
        .args(["-n", "hw.model"])
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_default();

    model_to_name(&model_id).unwrap_or(model_id)
}

fn model_to_name(model: &str) -> Option<String> {
    let version = if model.starts_with("Mac") && !model.starts_with("MacBook") && !model.starts_with("Macmini") && !model.starts_with("MacPro") {
        Some(model.strip_prefix("Mac")?)
    } else {
        None
    };


    // Database stolen from https://github.com/fastfetch-cli/fastfetch/blob/dev/src/detection/host/host_mac.c
    // Macbook Pro: https://support.apple.com/en-us/HT201300
    // Macbook Air: https://support.apple.com/en-us/HT201862
    // Mac mini:    https://support.apple.com/en-us/HT201894
    // iMac:        https://support.apple.com/en-us/HT201634
    // Mac Pro:     https://support.apple.com/en-us/HT202888
    // Mac Studio:  https://support.apple.com/en-us/HT213073
    if let Some(v) = version {
        return Some(match v {
            // MacBook Air
            "16,13" => "MacBook Air (15-inch, M4, 2025)",
            "16,12" => "MacBook Air (13-inch, M4, 2025)",
            "15,13" => "MacBook Air (15-inch, M3, 2024)",
            "15,12" => "MacBook Air (13-inch, M3, 2024)",
            "14,15" => "MacBook Air (15-inch, M2, 2023)",
            "14,2" => "MacBook Air (M2, 2022)",
            // MacBook Pro
            "16,8" => "MacBook Pro (16-inch, M4 Max, 2024)",
            "16,7" => "MacBook Pro (16-inch, M4 Pro, 2024)",
            "16,6" => "MacBook Pro (14-inch, M4 Max, 2024)",
            "16,5" => "MacBook Pro (14-inch, M4 Pro, 2024)",
            "16,1" => "MacBook Pro (14-inch, M4, 2024)",
            "15,11" => "MacBook Pro (16-inch, M3 Max, 2023)",
            "15,9" => "MacBook Pro (16-inch, M3 Pro, 2023)",
            "15,10" => "MacBook Pro (14-inch, M3 Max, 2023)",
            "15,8" => "MacBook Pro (14-inch, M3 Pro, 2023)",
            "15,6" | "15,7" => "MacBook Pro (16-inch, M2 Max, 2023)",
            "15,3" => "MacBook Pro (14-inch, M2 Max, 2023)",
            "14,10" | "14,6" => "MacBook Pro (16-inch, M2 Pro/Max, 2023)",
            "14,9" | "14,5" => "MacBook Pro (14-inch, M2 Pro/Max, 2023)",
            "14,7" => "MacBook Pro (13-inch, M2, 2022)",
            // Mac Studio
            "14,14" => "Mac Studio (M2 Ultra, 2023)",
            "14,13" => "Mac Studio (M2 Max, 2023)",
            "13,2" => "Mac Studio (M1 Ultra, 2022)",
            "13,1" => "Mac Studio (M1 Max, 2022)",
            // Mac mini
            "16,10" => "Mac mini (M4, 2024)",
            "16,3" => "Mac mini (M4 Pro, 2024)",
            "14,12" => "Mac mini (M2 Pro, 2023)",
            "14,3" => "Mac mini (M2, 2023)",
            // Mac Pro
            "14,8" => "Mac Pro (M2 Ultra, 2023)",
            // iMac
            "16,2" => "iMac (24-inch, M4, 2024)",
            "15,4" | "15,5" => "iMac (24-inch, M3, 2023)",
            _ => return None,
        }.to_string());
    }

    // Older Macs with specific prefixes
    Some(match model {
        // MacBook Air (Intel/M1)
        "MacBookAir10,1" => "MacBook Air (M1, 2020)",
        "MacBookAir9,1" => "MacBook Air (Retina, 13-inch, 2020)",
        "MacBookAir8,2" => "MacBook Air (Retina, 13-inch, 2019)",
        "MacBookAir8,1" => "MacBook Air (Retina, 13-inch, 2018)",
        // MacBook Pro (Intel/M1)
        "MacBookPro18,4" => "MacBook Pro (14-inch, M1 Max, 2021)",
        "MacBookPro18,3" => "MacBook Pro (14-inch, M1 Pro, 2021)",
        "MacBookPro18,2" => "MacBook Pro (16-inch, M1 Max, 2021)",
        "MacBookPro18,1" => "MacBook Pro (16-inch, M1 Pro, 2021)",
        "MacBookPro17,1" => "MacBook Pro (13-inch, M1, 2020)",
        "MacBookPro16,4" => "MacBook Pro (16-inch, 2019)",
        "MacBookPro16,3" => "MacBook Pro (13-inch, 2020, Two Thunderbolt 3 ports)",
        "MacBookPro16,2" => "MacBook Pro (13-inch, 2020, Four Thunderbolt 3 ports)",
        "MacBookPro16,1" => "MacBook Pro (16-inch, 2019)",
        "MacBookPro15,4" => "MacBook Pro (13-inch, 2019, Two Thunderbolt 3 ports)",
        "MacBookPro15,3" => "MacBook Pro (15-inch, 2019)",
        "MacBookPro15,2" => "MacBook Pro (13-inch, 2018/2019, Four Thunderbolt 3 ports)",
        "MacBookPro15,1" => "MacBook Pro (15-inch, 2018/2019)",
        // Mac mini
        "Macmini9,1" => "Mac mini (M1, 2020)",
        "Macmini8,1" => "Mac mini (2018)",
        // iMac
        "iMac21,2" | "iMac21,1" => "iMac (24-inch, M1, 2021)",
        "iMac20,2" | "iMac20,1" => "iMac (Retina 5K, 27-inch, 2020)",
        "iMac19,2" => "iMac (Retina 4K, 21.5-inch, 2019)",
        "iMac19,1" => "iMac (Retina 5K, 27-inch, 2019)",
        // Mac Pro
        "MacPro7,1" => "Mac Pro (2019)",
        "MacPro6,1" => "Mac Pro (Late 2013)",
        _ => return None,
    }.to_string())
}
