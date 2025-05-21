// 注意力机制 - 练习1
//
// 点积注意力（Scaled Dot-Product Attention）是Transformer架构的核心组件。
// 本练习将帮助你理解和实现基本的注意力计算过程。
//
// 任务：
// 1. 实现点积注意力的计算函数
// 2. 实现softmax函数
// 3. 实现注意力输出的计算

use std::f32;

// 我们使用前面练习中实现的Tensor结构
#[derive(Clone, Debug, PartialEq)]
pub struct Tensor {
    data: Vec<f32>,
    shape: Vec<usize>,
}

impl Tensor {
    // 创建指定形状的零张量
    pub fn zeros(shape: Vec<usize>) -> Self {
        let size = shape.iter().product();
        Tensor {
            data: vec![0.0; size],
            shape,
        }
    }

    // 获取张量的形状
    pub fn shape(&self) -> &Vec<usize> {
        &self.shape
    }

    // 获取张量的总元素数
    pub fn size(&self) -> usize {
        self.data.len()
    }

    // 获取张量在指定索引处的值
    pub fn get(&self, indices: &[usize]) -> Result<f32, String> {
        if indices.len() != self.shape.len() {
            return Err(format!(
                "索引维度 {} 与张量维度 {} 不匹配",
                indices.len(),
                self.shape.len()
            ));
        }

        for (i, &idx) in indices.iter().enumerate() {
            if idx >= self.shape[i] {
                return Err(format!(
                    "索引 {:?} 超出张量形状 {:?} 的范围",
                    indices, self.shape
                ));
            }
        }

        let flat_index = self.flatten_index(indices);
        Ok(self.data[flat_index])
    }

    // 设置张量在指定索引处的值
    pub fn set(&mut self, indices: &[usize], value: f32) -> Result<(), String> {
        if indices.len() != self.shape.len() {
            return Err(format!(
                "索引维度 {} 与张量维度 {} 不匹配",
                indices.len(),
                self.shape.len()
            ));
        }

        for (i, &idx) in indices.iter().enumerate() {
            if idx >= self.shape[i] {
                return Err(format!(
                    "索引 {:?} 超出张量形状 {:?} 的范围",
                    indices, self.shape
                ));
            }
        }

        let flat_index = self.flatten_index(indices);
        self.data[flat_index] = value;
        Ok(())
    }

    // 将多维索引转换为一维数组的索引
    fn flatten_index(&self, indices: &[usize]) -> usize {
        let mut flat_index = 0;
        let mut stride = 1;

        for i in (0..indices.len()).rev() {
            flat_index += indices[i] * stride;
            stride *= self.shape[i];
        }

        flat_index
    }

    // 矩阵乘法（用于注意力计算）
    pub fn matmul(&self, other: &Tensor) -> Result<Tensor, String> {
        // 检查是否为2D张量
        if self.shape.len() != 2 || other.shape.len() != 2 {
            return Err("矩阵乘法仅支持2D张量".to_string());
        }

        // 检查维度是否匹配
        if self.shape[1] != other.shape[0] {
            return Err(format!(
                "矩阵维度不匹配：({}, {}) 和 ({}, {})",
                self.shape[0], self.shape[1], other.shape[0], other.shape[1]
            ));
        }

        let m = self.shape[0];
        let n = other.shape[1];
        let k = self.shape[1]; // 或 other.shape[0]

        let mut result = Tensor::zeros(vec![m, n]);

        for i in 0..m {
            for j in 0..n {
                let mut sum = 0.0;
                for p in 0..k {
                    sum += self.get(&[i, p]).unwrap() * other.get(&[p, j]).unwrap();
                }
                result.set(&[i, j], sum).unwrap();
            }
        }

        Ok(result)
    }
}

// 注意力计算相关函数
pub struct AttentionFunctions;

impl AttentionFunctions {
    // TODO: 实现softmax函数
    // softmax(x_i) = exp(x_i) / sum(exp(x_j))
    pub fn softmax(x: &Tensor) -> Result<Tensor, String> {
        // 在这里实现函数
        // 提示：对每一行分别计算softmax
        unimplemented!("实现softmax函数");
    }

    // TODO: 实现点积注意力计算
    // attention(Q, K, V) = softmax(Q * K^T / sqrt(d_k)) * V
    // 其中，Q、K、V分别是查询、键、值矩阵，d_k是键的维度
    pub fn scaled_dot_product_attention(
        query: &Tensor,
        key: &Tensor,
        value: &Tensor,
    ) -> Result<Tensor, String> {
        // 在这里实现函数
        // 步骤：
        // 1. 计算注意力分数：Q * K^T
        // 2. 缩放：除以sqrt(d_k)
        // 3. 应用softmax
        // 4. 与V相乘得到输出
        unimplemented!("实现点积注意力计算");
    }
}

// 不要修改测试代码
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_softmax() {
        // 创建一个2x3的张量
        let mut x = Tensor::zeros(vec![2, 3]);
        x.set(&[0, 0], 1.0).unwrap();
        x.set(&[0, 1], 2.0).unwrap();
        x.set(&[0, 2], 3.0).unwrap();
        x.set(&[1, 0], 4.0).unwrap();
        x.set(&[1, 1], 5.0).unwrap();
        x.set(&[1, 2], 6.0).unwrap();

        let result = AttentionFunctions::softmax(&x).unwrap();

        // 第一行：softmax([1.0, 2.0, 3.0])
        // exp(1.0) = 2.718, exp(2.0) = 7.389, exp(3.0) = 20.086
        // sum = 30.193
        // softmax = [0.090, 0.245, 0.665]
        assert!((result.get(&[0, 0]).unwrap() - 0.090).abs() < 0.001);
        assert!((result.get(&[0, 1]).unwrap() - 0.245).abs() < 0.001);
        assert!((result.get(&[0, 2]).unwrap() - 0.665).abs() < 0.001);

        // 第二行：softmax([4.0, 5.0, 6.0])
        assert!((result.get(&[1, 0]).unwrap() - 0.090).abs() < 0.001);
        assert!((result.get(&[1, 1]).unwrap() - 0.245).abs() < 0.001);
        assert!((result.get(&[1, 2]).unwrap() - 0.665).abs() < 0.001);

        // 检查每行和为1
        let row1_sum = result.get(&[0, 0]).unwrap() + result.get(&[0, 1]).unwrap() + result.get(&[0, 2]).unwrap();
        let row2_sum = result.get(&[1, 0]).unwrap() + result.get(&[1, 1]).unwrap() + result.get(&[1, 2]).unwrap();
        assert!((row1_sum - 1.0).abs() < 0.001);
        assert!((row2_sum - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_scaled_dot_product_attention() {
        // 创建查询矩阵 Q (2x3)
        let mut query = Tensor::zeros(vec![2, 3]);
        query.set(&[0, 0], 1.0).unwrap();
        query.set(&[0, 1], 0.0).unwrap();
        query.set(&[0, 2], 0.0).unwrap();
        query.set(&[1, 0], 0.0).unwrap();
        query.set(&[1, 1], 1.0).unwrap();
        query.set(&[1, 2], 0.0).unwrap();

        // 创建键矩阵 K (2x3)
        let mut key = Tensor::zeros(vec![2, 3]);
        key.set(&[0, 0], 1.0).unwrap();
        key.set(&[0, 1], 0.0).unwrap();
        key.set(&[0, 2], 0.0).unwrap();
        key.set(&[1, 0], 0.0).unwrap();
        key.set(&[1, 1], 1.0).unwrap();
        key.set(&[1, 2], 0.0).unwrap();

        // 创建值矩阵 V (2x2)
        let mut value = Tensor::zeros(vec![2, 2]);
        value.set(&[0, 0], 1.0).unwrap();
        value.set(&[0, 1], 2.0).unwrap();
        value.set(&[1, 0], 3.0).unwrap();
        value.set(&[1, 1], 4.0).unwrap();

        let result = AttentionFunctions::scaled_dot_product_attention(&query, &key, &value).unwrap();

        // 验证结果形状
        assert_eq!(result.shape(), &vec![2, 2]);

        // 简化的注意力计算（对角线注意力矩阵）应该接近于原始值矩阵
        // 但由于softmax和缩放，会有一些差异
        assert!((result.get(&[0, 0]).unwrap() - 1.0).abs() < 0.1);
        assert!((result.get(&[0, 1]).unwrap() - 2.0).abs() < 0.1);
        assert!((result.get(&[1, 0]).unwrap() - 3.0).abs() < 0.1);
        assert!((result.get(&[1, 1]).unwrap() - 4.0).abs() < 0.1);
    }
}