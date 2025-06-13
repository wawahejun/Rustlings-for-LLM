// 在大模型训练中，f16的精度损失和数值溢出是常见问题。
// 在这个练习中，我们将实现f16的溢出检测和处理机制。

// f16的常量定义
const F16_MAX: f32 = 65504.0; // f16最大正数
const F16_MIN_NORMAL: f32 = 6.104e-5; // f16最小正规数

// 辅助函数：简化的f32到f16位模式的转换（仅用于测试）
// 注意：这个函数不需要实现，已经提供
fn f32_to_f16_bits(x: f32) -> u16 {
    if x == 0.0 {
        return 0; // 零
    }
    
    let bits = x.to_bits();
    let sign = (bits >> 31) & 1;
    let exp = ((bits >> 23) & 0xFF) as i32 - 127 + 15;
    let frac = (bits & 0x7FFFFF) >> 13;
    
    if exp >= 31 {
        return (sign << 15 | 0x7C00) as u16; // 无穷大
    } else if exp <= 0 {
        return (sign << 15) as u16; // 下溢到零
    }
    
    ((sign << 15) | ((exp as u32) << 10) | frac) as u16
}

// 辅助函数：简化的f16位模式到f32的转换（仅用于测试）
// 注意：这个函数不需要实现，已经提供
fn f16_bits_to_f32(bits: u16) -> f32 {
    let sign = ((bits >> 15) & 1) as u32;
    let exp = ((bits >> 10) & 0x1F) as i32;
    let frac = (bits & 0x3FF) as u32;
    
    if exp == 0 {
        return 0.0; // 零或非规格化数（简化为零）
    } else if exp == 31 {
        return if frac == 0 {
            if sign == 0 { f32::INFINITY } else { f32::NEG_INFINITY }
        } else {
            f32::NAN
        };
    }
    
    let adjusted_exp = (exp - 15 + 127) as u32;
    let f32_bits = (sign << 31) | (adjusted_exp << 23) | (frac << 13);
    f32::from_bits(f32_bits)
}

// TODO: 实现f16数值范围检查
// 检查f32数值是否在f16表示范围内
// 返回：数值是否在f16可表示范围内
fn check_f16_range(x: f32) -> bool {
    // 提示：检查x的绝对值是否在[F16_MIN_NORMAL, F16_MAX]范围内，或者x是否为0
    unimplemented!("请实现f16范围检查")
}

// TODO: 实现f16下溢处理
// 当数值绝对值小于f16最小正规数时，将其设为0
// 当数值在范围内时，保持原值
// 返回：处理后的f16位模式
fn handle_f16_underflow(x: f32) -> u16 {
    // 提示：使用提供的f32_to_f16_bits辅助函数
    unimplemented!("请实现f16下溢处理")
}

// TODO: 实现f16上溢处理
// 当数值绝对值大于f16最大值时，将其设为对应符号的无穷大
// 当数值在范围内时，保持原值
// 返回：处理后的f16位模式
fn handle_f16_overflow(x: f32) -> u16 {
    // 提示：使用提供的f32_to_f16_bits辅助函数
    unimplemented!("请实现f16上溢处理")
}

// TODO: 实现f16精度损失检测
// 计算f32转f16再转回f32的相对误差
// 返回：相对误差是否超过给定阈值
fn check_precision_loss(x: f32, threshold: f32) -> bool {
    // 提示：
    // 1. 使用提供的f32_to_f16_bits和f16_bits_to_f32辅助函数
    // 2. 计算相对误差 = |原值 - 转换后的值| / |原值|
    // 3. 对于接近0的值，需要特殊处理以避免除以0
    unimplemented!("请实现精度损失检测")
}

#[cfg(test)]
mod tests {
    use super::*;

    // f16位模式常量
    const F16_INFINITY: u16 = 0x7C00; // f16正无穷大
    const F16_NEG_INFINITY: u16 = 0xFC00; // f16负无穷大
    const F16_ZERO: u16 = 0x0000; // f16零
    const F16_NEG_ZERO: u16 = 0x8000; // f16负零

    #[test]
    fn test_f16_range_check() {
        // 测试正常范围内的值
        assert!(check_f16_range(1.0), "1.0应该在f16范围内");
        assert!(check_f16_range(-1.0), "-1.0应该在f16范围内");
        assert!(check_f16_range(F16_MAX), "F16_MAX应该在f16范围内");
        assert!(check_f16_range(-F16_MAX), "-F16_MAX应该在f16范围内");
        assert!(check_f16_range(F16_MIN_NORMAL), "F16_MIN_NORMAL应该在f16范围内");
        assert!(check_f16_range(-F16_MIN_NORMAL), "-F16_MIN_NORMAL应该在f16范围内");
        
        // 测试零值
        assert!(check_f16_range(0.0), "0.0应该在f16范围内");
        assert!(check_f16_range(-0.0), "-0.0应该在f16范围内");

        // 测试超出范围的值
        assert!(!check_f16_range(F16_MAX + 1.0), "F16_MAX + 1.0应该超出f16范围");
        assert!(!check_f16_range(-(F16_MAX + 1.0)), "-(F16_MAX + 1.0)应该超出f16范围");
        assert!(!check_f16_range(F16_MIN_NORMAL * 0.9), "小于F16_MIN_NORMAL的值应该超出f16范围");
        assert!(!check_f16_range(-F16_MIN_NORMAL * 0.9), "小于-F16_MIN_NORMAL的值应该超出f16范围");
    }

    #[test]
    fn test_f16_underflow() {
        // 测试下溢到零的情况
        let tiny_positive = F16_MIN_NORMAL * 0.5;
        assert_eq!(handle_f16_underflow(tiny_positive), F16_ZERO, 
                  "小于最小正规数的正值应该下溢到0");
        
        let tiny_negative = -F16_MIN_NORMAL * 0.5;
        assert_eq!(handle_f16_underflow(tiny_negative), F16_NEG_ZERO, 
                  "小于最小正规数的负值应该下溢到-0");

        // 测试零值处理
        assert_eq!(handle_f16_underflow(0.0), F16_ZERO, 
                  "0.0应该被正确处理为零");
        assert_eq!(handle_f16_underflow(-0.0), F16_NEG_ZERO, 
                  "-0.0应该被正确处理为负零");

        // 测试正常范围内的值不应下溢
        assert_ne!(handle_f16_underflow(1.0), F16_ZERO, 
                  "正常值不应该下溢到0");
        assert_ne!(handle_f16_underflow(F16_MIN_NORMAL), F16_ZERO, 
                  "最小正规数不应该下溢到0");
        assert_ne!(handle_f16_underflow(-F16_MIN_NORMAL), F16_NEG_ZERO, 
                  "负最小正规数不应该下溢到-0");
    }

    #[test]
    fn test_f16_overflow() {
        // 测试上溢到无穷大的情况
        let huge_positive = F16_MAX * 1.1;
        assert_eq!(handle_f16_overflow(huge_positive), F16_INFINITY, 
                  "超过最大值的正数应该上溢到正无穷大");
        
        let huge_negative = -F16_MAX * 1.1;
        assert_eq!(handle_f16_overflow(huge_negative), F16_NEG_INFINITY, 
                  "超过最大值的负数应该上溢到负无穷大");

        // 测试正常范围内的值不应上溢
        assert_ne!(handle_f16_overflow(1.0), F16_INFINITY, 
                  "正常值不应该上溢到无穷大");
        assert_ne!(handle_f16_overflow(-1.0), F16_NEG_INFINITY, 
                  "正常值不应该上溢到负无穷大");
        assert_ne!(handle_f16_overflow(F16_MAX), F16_INFINITY, 
                  "最大值不应该上溢到无穷大");
        assert_ne!(handle_f16_overflow(-F16_MAX), F16_NEG_INFINITY, 
                  "负最大值不应该上溢到负无穷大");
    }

    #[test]
    fn test_precision_loss() {
        // 测试有显著精度损失的情况
        // 大数精度损失 - 接近最大值的数
        assert!(check_precision_loss(F16_MAX * 0.999, 0.0001), 
                "接近最大值的数应该有显著精度损失");

        // 小数精度损失 - 非2的幂次
        assert!(check_precision_loss(0.333333, 0.0001), 
                "非2的幂次小数应该有显著精度损失");
        assert!(check_precision_loss(1.0/3.0, 0.0001), 
                "1/3应该有显著精度损失");

        // 测试精度损失不显著的情况
        // 整数（应该没有精度损失）
        assert!(!check_precision_loss(42.0, 0.0001), 
                "整数不应该有显著精度损失");

        // 2的幂（应该没有精度损失）
        assert!(!check_precision_loss(256.0, 0.0001), 
                "2的幂次不应该有显著精度损失");
        assert!(!check_precision_loss(1.0, 0.0001), 
                "1.0不应该有显著精度损失");
        
        // 测试不同阈值的影响
        assert!(check_precision_loss(0.1, 0.00001), 
                "使用较小阈值时，0.1应该被检测为有精度损失");
        assert!(!check_precision_loss(0.1, 0.01), 
                "使用较大阈值时，0.1应该被检测为无显著精度损失");
    }

    #[test]
    fn test_edge_cases() {
        // 测试零值的处理
        assert!(check_f16_range(0.0), "0.0应该在f16范围内");
        assert_eq!(handle_f16_underflow(0.0), F16_ZERO, "0.0应该被正确处理");
        assert_eq!(handle_f16_overflow(0.0), F16_ZERO, "0.0不应该上溢");
        assert!(!check_precision_loss(0.0, 0.0001), "0.0不应该有精度损失");

        // 测试接近边界值的处理
        // 接近最大值
        let almost_max = F16_MAX - 0.1;
        assert!(check_f16_range(almost_max), "接近最大值的数应该在范围内");
        assert_ne!(handle_f16_overflow(almost_max), F16_INFINITY, "接近最大值的数不应该上溢");
        
        // 接近最小值
        let almost_min = F16_MIN_NORMAL * 1.01;
        assert!(check_f16_range(almost_min), "接近最小值的数应该在范围内");
        assert_ne!(handle_f16_underflow(almost_min), F16_ZERO, "接近最小值的数不应该下溢");
        
        // 测试特殊值的精度损失
        // 非常小但在范围内的值
        let small_in_range = F16_MIN_NORMAL * 1.5;
        assert!(check_precision_loss(small_in_range, 0.0001), 
                "非常小但在范围内的值应该有精度损失");
    }
    
    #[test]
    fn test_conversion_helpers() {
        // 测试辅助函数的正确性
        // 测试正常值的转换
        let f32_val = 1.0;
        let f16_bits = f32_to_f16_bits(f32_val);
        let f32_back = f16_bits_to_f32(f16_bits);
        assert!((f32_val - f32_back).abs() < 0.0001, 
                "正常值的往返转换应该精确");
        
        // 测试零值的转换
        assert_eq!(f32_to_f16_bits(0.0), F16_ZERO, 
                  "0.0应该转换为f16零");
        assert_eq!(f16_bits_to_f32(F16_ZERO), 0.0, 
                  "f16零应该转换为0.0");
        
        // 测试无穷大的转换
        assert_eq!(f16_bits_to_f32(F16_INFINITY), f32::INFINITY, 
                  "f16正无穷大应该转换为f32正无穷大");
        assert_eq!(f16_bits_to_f32(F16_NEG_INFINITY), f32::NEG_INFINITY, 
                  "f16负无穷大应该转换为f32负无穷大");
    }
}