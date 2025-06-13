// 在大模型训练中，使用f16（半精度浮点数）可以显著减少内存使用和提高计算速度。
// 在这个练习中，我们将学习f16的基本位操作和简单转换，不使用half库。

// f16的位模式：1位符号位，5位指数位，10位尾数位
// 总共16位，按照IEEE 754-2008标准

// TODO: 提取f16的符号位
// 输入：f16的位表示（u16）
// 输出：符号位（0表示正数，1表示负数）
fn extract_f16_sign(bits: u16) -> u8 {
    // 提示：符号位是最高位（第15位）
    unimplemented!("请实现符号位提取")
}

// TODO: 提取f16的指数位
// 输入：f16的位表示（u16）
// 输出：指数位的值（5位，范围0-31）
fn extract_f16_exponent(bits: u16) -> u8 {
    // 提示：指数位是第10-14位
    unimplemented!("请实现指数位提取")
}

// TODO: 提取f16的尾数位
// 输入：f16的位表示（u16）
// 输出：尾数位的值（10位，范围0-1023）
fn extract_f16_mantissa(bits: u16) -> u16 {
    // 提示：尾数位是第0-9位
    unimplemented!("请实现尾数位提取")
}

// TODO: 构造f16的位表示
// 输入：符号位、指数位、尾数位
// 输出：f16的完整位表示
fn construct_f16_bits(sign: u8, exponent: u8, mantissa: u16) -> u16 {
    // 提示：将三个部分组合成16位的值
    unimplemented!("请实现f16位构造")
}

// TODO: 简化的f16到f32转换（仅处理正常数值）
// 输入：f16的位表示
// 输出：对应的f32值
// 注意：这里只需要处理正常的数值，不需要处理特殊值（无穷大、NaN等）
fn simple_f16_to_f32(bits: u16) -> f32 {
    // 提示：
    // 1. 提取符号、指数、尾数
    // 2. 调整指数偏移（f16偏移15，f32偏移127）
    // 3. 构造f32的位表示并转换
    unimplemented!("请实现简化的f16到f32转换")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_f16_sign() {
        // 测试正数的符号位
        let positive_bits = 0b0011110000000000u16; // 正数
        assert_eq!(extract_f16_sign(positive_bits), 0, "正数符号位应该是0");

        // 测试负数的符号位
        let negative_bits = 0b1011110000000000u16; // 负数
        assert_eq!(extract_f16_sign(negative_bits), 1, "负数符号位应该是1");

        // 测试零的符号位
        let zero_bits = 0b0000000000000000u16;
        assert_eq!(extract_f16_sign(zero_bits), 0, "零的符号位应该是0");
    }

    #[test]
    fn test_extract_f16_exponent() {
        // 测试指数位提取
        let bits1 = 0b0011110000000000u16; // 指数位为01111 (15)
        assert_eq!(extract_f16_exponent(bits1), 15, "指数位提取错误");

        let bits2 = 0b0100000000000000u16; // 指数位为10000 (16)
        assert_eq!(extract_f16_exponent(bits2), 16, "指数位提取错误");

        let bits3 = 0b0000000000000000u16; // 指数位为00000 (0)
        assert_eq!(extract_f16_exponent(bits3), 0, "指数位提取错误");
    }

    #[test]
    fn test_extract_f16_mantissa() {
        // 测试尾数位提取
        let bits1 = 0b0000000000000001u16; // 尾数位为0000000001 (1)
        assert_eq!(extract_f16_mantissa(bits1), 1, "尾数位提取错误");

        let bits2 = 0b0000001111111111u16; // 尾数位为1111111111 (1023)
        assert_eq!(extract_f16_mantissa(bits2), 1023, "尾数位提取错误");

        let bits3 = 0b1111110000000000u16; // 尾数位为0000000000 (0)
        assert_eq!(extract_f16_mantissa(bits3), 0, "尾数位提取错误");
    }

    #[test]
    fn test_construct_f16_bits() {
        // 测试位构造
        let result1 = construct_f16_bits(0, 15, 0);
        assert_eq!(result1, 0b0011110000000000u16, "f16位构造错误");

        let result2 = construct_f16_bits(1, 16, 512);
        assert_eq!(result2, 0b1100001000000000u16, "f16位构造错误");

        let result3 = construct_f16_bits(0, 0, 1023);
        assert_eq!(result3, 0b0000001111111111u16, "f16位构造错误");
    }

    #[test]
    fn test_simple_f16_to_f32() {
        // 测试1.0的转换 (f16: 0x3C00)
        let f16_one = 0x3C00u16;
        let result = simple_f16_to_f32(f16_one);
        assert!((result - 1.0).abs() < 0.0001, "1.0转换失败");

        // 测试2.0的转换 (f16: 0x4000)
        let f16_two = 0x4000u16;
        let result = simple_f16_to_f32(f16_two);
        assert!((result - 2.0).abs() < 0.0001, "2.0转换失败");

        // 测试0.5的转换 (f16: 0x3800)
        let f16_half = 0x3800u16;
        let result = simple_f16_to_f32(f16_half);
        assert!((result - 0.5).abs() < 0.0001, "0.5转换失败");

        // 测试负数-1.0的转换 (f16: 0xBC00)
        let f16_neg_one = 0xBC00u16;
        let result = simple_f16_to_f32(f16_neg_one);
        assert!((result + 1.0).abs() < 0.0001, "-1.0转换失败");
    }

    #[test]
    fn test_bit_operations_integration() {
        // 综合测试：分解再重组
        let original = 0x4200u16; // 3.0
        let sign = extract_f16_sign(original);
        let exponent = extract_f16_exponent(original);
        let mantissa = extract_f16_mantissa(original);
        let reconstructed = construct_f16_bits(sign, exponent, mantissa);
        assert_eq!(original, reconstructed, "位操作综合测试失败");
    }
}