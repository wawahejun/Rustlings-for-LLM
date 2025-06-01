// 在大模型训练中，f16的精度损失和数值溢出是常见问题。
// 在这个练习中，我们将实现f16的溢出检测和处理机制。

// TODO: 实现f16数值范围检查
// 检查f32数值是否在f16表示范围内
// f16范围：[-65504, 65504]，最小正规数：6.104e-5
fn check_f16_range(x: f32) -> bool {
    unimplemented!("请实现f16范围检查")
}

// TODO: 实现f16下溢处理
// 当数值小于f16最小正规数时，返回0
// 当数值在范围内时，返回原值的f16表示
fn handle_f16_underflow(x: f32) -> u16 {
    unimplemented!("请实现f16下溢处理")
}

// TODO: 实现f16上溢处理
// 当数值大于f16最大值时，返回无穷大
// 当数值在范围内时，返回原值的f16表示
fn handle_f16_overflow(x: f32) -> u16 {
    unimplemented!("请实现f16上溢处理")
}

// TODO: 实现f16精度损失检测
// 计算f32转f16再转回f32的相对误差
// 返回相对误差是否超过阈值
fn check_precision_loss(x: f32, threshold: f32) -> bool {
    unimplemented!("请实现精度损失检测")
}

#[cfg(test)]
mod tests {
    use super::*;

    // f16位模式常量
    const F16_INFINITY: u16 = 0x7C00; // f16正无穷大
    const F16_ZERO: u16 = 0x0000; // f16零

    #[test]
    fn test_f16_range_check() {
        // 测试正常范围
        assert!(check_f16_range(1.0), "1.0应该在f16范围内");
        assert!(check_f16_range(-1.0), "-1.0应该在f16范围内");
        assert!(check_f16_range(65504.0), "65504.0应该在f16范围内");
        assert!(check_f16_range(-65504.0), "-65504.0应该在f16范围内");

        // 测试超出范围
        assert!(!check_f16_range(65505.0), "65505.0应该超出f16范围");
        assert!(!check_f16_range(-65505.0), "-65505.0应该超出f16范围");
        assert!(!check_f16_range(6.103e-5), "6.103e-5应该超出f16范围");
    }

    #[test]
    fn test_f16_underflow() {
        // 测试下溢
        let result = handle_f16_underflow(1e-6);
        assert_eq!(result, F16_ZERO, "极小值应该下溢到0");

        // 测试正常值
        let result = handle_f16_underflow(1.0);
        assert_ne!(result, F16_ZERO, "正常值不应该下溢到0");

        // 测试最小正规数
        let result = handle_f16_underflow(6.104e-5);
        assert_ne!(result, F16_ZERO, "最小正规数不应该下溢到0");
    }

    #[test]
    fn test_f16_overflow() {
        // 测试上溢
        let result = handle_f16_overflow(70000.0);
        assert_eq!(result, F16_INFINITY, "70000.0应该上溢到无穷大");

        // 测试正常值
        let result = handle_f16_overflow(1.0);
        assert_ne!(result, F16_INFINITY, "1.0不应该上溢到无穷大");

        // 测试最大值
        let result = handle_f16_overflow(65504.0);
        assert_ne!(result, F16_INFINITY, "65504.0不应该上溢到无穷大");
    }

    #[test]
    fn test_precision_loss() {
        // 测试大数精度损失
        assert!(check_precision_loss(65504.0, 0.001), 
                "65504.0应该有显著精度损失");

        // 测试小数精度损失
        assert!(check_precision_loss(0.333333, 0.001), 
                "0.333333应该有显著精度损失");

        // 测试整数（应该没有精度损失）
        assert!(!check_precision_loss(42.0, 0.001), 
                "42.0不应该有显著精度损失");

        // 测试2的幂（应该没有精度损失）
        assert!(!check_precision_loss(256.0, 0.001), 
                "256.0不应该有显著精度损失");
    }

    #[test]
    fn test_edge_cases() {
        // 测试0的处理
        assert!(check_f16_range(0.0), "0.0应该在f16范围内");
        let result = handle_f16_underflow(0.0);
        assert_eq!(result, F16_ZERO, "0.0应该被正确处理");

        // 测试接近最大值的数
        let almost_max = 65503.9;
        assert!(check_f16_range(almost_max), "接近最大值的数应该在范围内");
        let result = handle_f16_overflow(almost_max);
        assert_ne!(result, F16_INFINITY, "接近最大值的数不应该上溢");

        // 测试接近最小值的数
        let almost_min = 6.105e-5;
        assert!(check_f16_range(almost_min), "接近最小值的数应该在范围内");
        let result = handle_f16_underflow(almost_min);
        assert_ne!(result, F16_ZERO, "接近最小值的数不应该下溢");
    }
}