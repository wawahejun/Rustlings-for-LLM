// 在大模型训练中，使用f16（半精度浮点数）可以显著减少内存使用和提高计算速度。
// 在这个练习中，我们将实现f16的基本表示和运算，不使用half库。

// f16的位模式：1位符号位，5位指数位，10位尾数位
// 总共16位，按照IEEE 754-2008标准

// TODO: 实现从f32到f16的转换
// 需要处理：
// 1. 提取符号位
// 2. 处理指数偏移（f32是127，f16是15）
// 3. 舍入尾数（从23位到10位）
fn f32_to_f16_bits(x: f32) -> u16 {
    unimplemented!("请实现f32到f16的转换")
}

// TODO: 实现从f16到f32的转换
// 需要处理：
// 1. 保持符号位
// 2. 调整指数偏移
// 3. 扩展尾数（从10位到23位）
fn f16_bits_to_f32(x: u16) -> f32 {
    unimplemented!("请实现f16到f32的转换")
}

// TODO: 实现f16加法
// 输入和输出都是f16的位模式
fn f16_add(a: u16, b: u16) -> u16 {
    unimplemented!("请实现f16加法")
}

// TODO: 实现f16乘法
// 输入和输出都是f16的位模式
fn f16_multiply(a: u16, b: u16) -> u16 {
    unimplemented!("请实现f16乘法")
}

#[cfg(test)]
mod tests {
    use super::*;

    const F16_EPSILON: f32 = 0.0001;

    #[test]
    fn test_f32_to_f16_conversion() {
        // 测试正数转换
        let x: f32 = 1.0;
        let bits = f32_to_f16_bits(x);
        let result = f16_bits_to_f32(bits);
        assert!((result - x).abs() < F16_EPSILON, "转换1.0失败");

        // 测试负数转换
        let x: f32 = -2.0;
        let bits = f32_to_f16_bits(x);
        let result = f16_bits_to_f32(bits);
        assert!((result - x).abs() < F16_EPSILON, "转换-2.0失败");

        // 测试小数转换
        let x: f32 = 0.333;
        let bits = f32_to_f16_bits(x);
        let result = f16_bits_to_f32(bits);
        assert!((result - x).abs() < F16_EPSILON, "转换0.333失败");
    }

    #[test]
    fn test_f16_addition() {
        // 1.0 + 2.0 = 3.0
        let a = f32_to_f16_bits(1.0);
        let b = f32_to_f16_bits(2.0);
        let result = f16_add(a, b);
        let sum = f16_bits_to_f32(result);
        assert!((sum - 3.0).abs() < F16_EPSILON, "1.0 + 2.0 != 3.0");

        // 0.5 + (-0.5) = 0.0
        let a = f32_to_f16_bits(0.5);
        let b = f32_to_f16_bits(-0.5);
        let result = f16_add(a, b);
        let sum = f16_bits_to_f32(result);
        assert!(sum.abs() < F16_EPSILON, "0.5 + (-0.5) != 0.0");
    }

    #[test]
    fn test_f16_multiplication() {
        // 2.0 * 3.0 = 6.0
        let a = f32_to_f16_bits(2.0);
        let b = f32_to_f16_bits(3.0);
        let result = f16_multiply(a, b);
        let product = f16_bits_to_f32(result);
        assert!((product - 6.0).abs() < F16_EPSILON, "2.0 * 3.0 != 6.0");

        // -2.0 * 3.0 = -6.0
        let a = f32_to_f16_bits(-2.0);
        let b = f32_to_f16_bits(3.0);
        let result = f16_multiply(a, b);
        let product = f16_bits_to_f32(result);
        assert!((product + 6.0).abs() < F16_EPSILON, "-2.0 * 3.0 != -6.0");
    }

    #[test]
    fn test_special_cases() {
        // 测试0的转换
        let zero_bits = f32_to_f16_bits(0.0);
        let zero_back = f16_bits_to_f32(zero_bits);
        assert!(zero_back.abs() < F16_EPSILON, "0.0转换失败");

        // 测试最大正数约65504
        let max_f16 = 65504.0;
        let max_bits = f32_to_f16_bits(max_f16);
        let max_back = f16_bits_to_f32(max_bits);
        assert!((max_back - max_f16).abs() / max_f16 < 0.001, "最大值转换失败");

        // 测试最小正规数约6.104e-5
        let min_normal = 6.104e-5;
        let min_bits = f32_to_f16_bits(min_normal);
        let min_back = f16_bits_to_f32(min_bits);
        assert!((min_back - min_normal).abs() / min_normal < 0.001, "最小正规数转换失败");
    }
}