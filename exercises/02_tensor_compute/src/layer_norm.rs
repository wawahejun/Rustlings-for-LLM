// 简化版层归一化练习
// 我们从最基础的1D向量开始，逐步理解层归一化的核心概念

// TODO: 计算向量的均值
// 输入: 一个f32向量
// 输出: 均值 (单个f32值)
fn compute_mean(x: &[f32]) -> f32 {
    // 提示: 使用 x.iter().sum::<f32>() / x.len() as f32
    unimplemented!("请实现均值计算")
}

// TODO: 计算向量的标准差
// 输入: 一个f32向量和它的均值
// 输出: 标准差 (单个f32值)
fn compute_std(x: &[f32], mean: f32) -> f32 {
    // 提示: sqrt(平均平方差)
    // 1. 计算每个元素与均值的差的平方
    // 2. 求平均值
    // 3. 开平方根
    unimplemented!("请实现标准差计算")
}

// TODO: 实现简单的层归一化
// 输入: 向量x, 缩放参数gamma, 偏移参数beta, 小常数epsilon
// 输出: 归一化后的向量
// 公式: (x - mean) / (std + epsilon) * gamma + beta
fn layer_norm(x: &[f32], gamma: f32, beta: f32, epsilon: f32) -> Vec<f32> {
    unimplemented!("请实现层归一化")
}

// TODO: 批量层归一化
// 对多个向量分别进行层归一化
// 输入: 多个向量组成的2D数组
// 输出: 归一化后的2D数组
fn batch_layer_norm(batch: &[Vec<f32>], gamma: f32, beta: f32, epsilon: f32) -> Vec<Vec<f32>> {
    unimplemented!("请实现批量层归一化")
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f32 = 1e-6;

    #[test]
    fn test_compute_mean() {
        // 简单测试用例
        let x = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let mean = compute_mean(&x);
        assert!((mean - 3.0).abs() < EPSILON, "均值应该是3.0，实际是{}", mean);

        // 边界测试
        let x = vec![0.0];
        let mean = compute_mean(&x);
        assert!((mean - 0.0).abs() < EPSILON, "单个元素的均值应该是该元素本身");
    }

    #[test]
    fn test_compute_std() {
        // 简单测试用例: [1,2,3,4,5] 均值=3, 标准差=sqrt(2)≈1.414
        let x = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let mean = 3.0;
        let std = compute_std(&x, mean);
        assert!((std - 1.414).abs() < 0.01, "标准差应该约为1.414，实际是{}", std);

        // 所有元素相同的情况
        let x = vec![5.0, 5.0, 5.0];
        let mean = 5.0;
        let std = compute_std(&x, mean);
        assert!((std - 0.0).abs() < EPSILON, "相同元素的标准差应该是0");
    }

    #[test]
    fn test_layer_norm() {
        // 测试基本归一化: 输入[1,2,3,4,5], gamma=1, beta=0
        // 归一化后应该均值≈0，标准差≈1
        let x = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = layer_norm(&x, 1.0, 0.0, 1e-8);
        
        // 检查结果的均值应该接近0
        let result_mean = compute_mean(&result);
        assert!(result_mean.abs() < 0.01, "归一化后均值应该接近0，实际是{}", result_mean);
        
        // 检查结果的标准差应该接近1
        let result_std = compute_std(&result, result_mean);
        assert!((result_std - 1.0).abs() < 0.01, "归一化后标准差应该接近1，实际是{}", result_std);

        // 测试缩放和偏移: gamma=2, beta=1
        let result2 = layer_norm(&x, 2.0, 1.0, 1e-8);
        let result2_mean = compute_mean(&result2);
        assert!((result2_mean - 1.0).abs() < 0.01, "应用beta=1后均值应该接近1");
    }

    #[test]
    fn test_batch_layer_norm() {
        let batch = vec![
            vec![1.0, 2.0, 3.0],
            vec![10.0, 20.0, 30.0],
            vec![100.0, 200.0, 300.0]
        ];
        
        let result = batch_layer_norm(&batch, 1.0, 0.0, 1e-8);
        
        // 检查每个向量都被正确归一化
        for (i, normalized) in result.iter().enumerate() {
            let mean = compute_mean(normalized);
            assert!(mean.abs() < 0.01, "第{}个向量归一化后均值应该接近0", i);
            
            let std = compute_std(normalized, mean);
            assert!((std - 1.0).abs() < 0.01, "第{}个向量归一化后标准差应该接近1", i);
        }
    }

    #[test]
    fn test_layer_norm_properties() {
        // 验证层归一化的重要性质
        let x = vec![10.0, 15.0, 20.0, 25.0, 30.0]; // 较大的数值
        let result = layer_norm(&x, 1.0, 0.0, 1e-8);
        
        // 性质1: 归一化后，相对顺序保持不变
        for i in 0..result.len()-1 {
            assert!(result[i] < result[i+1], "归一化后应该保持原有的大小顺序");
        }
        
        // 性质2: 输入加上常数，归一化结果不变
        let x_shifted: Vec<f32> = x.iter().map(|&v| v + 1000.0).collect();
        let result_shifted = layer_norm(&x_shifted, 1.0, 0.0, 1e-8);
        
        for (a, b) in result.iter().zip(result_shifted.iter()) {
            assert!((a - b).abs() < 0.001, "平移不变性: 输入加常数不应影响归一化结果");
        }
    }
}