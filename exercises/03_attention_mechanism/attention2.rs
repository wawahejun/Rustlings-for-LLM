// 注意力机制 - 练习2
//
// 多头注意力（Multi-Head Attention）是Transformer架构中的关键组件，
// 它允许模型同时关注不同位置的信息，从不同的表示子空间学习信息。
//
// 任务：
// 1. 实现线性投影函数（用于生成Q、K、V）
// 2. 实现多头注意力的计算过程
// 3. 实现多头输出的合并和最终线性变换

// 我们使用前面练习中实现的Tensor结构和基本的点积注意力函数
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

    // 创建随机初始化的张量（用于权重初始化）
    pub fn random(shape: Vec<usize>) -> Self {
        let size = shape.iter().product();
        let mut data = Vec::with_capacity(size);
        for _ in 0..size {
            // 简单的随机初始化，实际应用中会使用更复杂的初始化方法
            data.push(rand::random::<f32>() * 0.1);
        }
        Tensor { data, shape }
    }

    // 获取张量的形状
    pub fn shape(&self) -> &Vec<usize> {
        &self.shape
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

    // 矩阵乘法
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

    // 张量分割（用于多头注意力）
    // 将一个形状为[batch_size, seq_len, d_model]的张量分割成num_heads个形状为[batch_size, seq_len, d_head]的张量
    pub fn split_heads(&self, num_heads: usize) -> Result<Vec<Tensor>, String> {
        if self.shape.len() != 3 {
            return Err("分割头部操作需要3D张量".to_string());
        }

        let batch_size = self.shape[0];
        let seq_len = self.shape[1];
        let d_model = self.shape[2];

        if d_model % num_heads != 0 {
            return Err(format!(
                "模型维度 {} 不能被头数 {} 整除",
                d_model, num_heads
            ));
        }

        let d_head = d_model / num_heads;
        let mut heads = Vec::with_capacity(num_heads);

        for h in 0..num_heads {
            let mut head = Tensor::zeros(vec![batch_size, seq_len, d_head]);
            for b in 0..batch_size {
                for s in 0..seq_len {
                    for i in 0..d_head {
                        let value = self.get(&[b, s, h * d_head + i]).unwrap();
                        head.set(&[b, s, i], value).unwrap();
                    }
                }
            }
            heads.push(head);
        }

        Ok(heads)
    }

    // 张量合并（用于多头注意力的输出合并）
    // 将多个形状为[batch_size, seq_len, d_head]的张量合并成一个形状为[batch_size, seq_len, d_model]的张量
    pub fn concat_heads(heads: &[Tensor]) -> Result<Tensor, String> {
        if heads.is_empty() {
            return Err("没有头部可合并".to_string());
        }

        let num_heads = heads.len();
        let batch_size = heads[0].shape()[0];
        let seq_len = heads[0].shape()[1];
        let d_head = heads[0].shape()[2];

        // 检查所有头部的形状是否一致
        for (i, head) in heads.iter().enumerate() {
            if head.shape() != &vec![batch_size, seq_len, d_head] {
                return Err(format!(
                    "头部 {} 的形状 {:?} 与预期形状 {:?} 不匹配",
                    i,
                    head.shape(),
                    vec![batch_size, seq_len, d_head]
                ));
            }
        }

        let d_model = d_head * num_heads;
        let mut result = Tensor::zeros(vec![batch_size, seq_len, d_model]);

        for h in 0..num_heads {
            for b in 0..batch_size {
                for s in 0..seq_len {
                    for i in 0..d_head {
                        let value = heads[h].get(&[b, s, i]).unwrap();
                        result.set(&[b, s, h * d_head + i], value).unwrap();
                    }
                }
            }
        }

        Ok(result)
    }
}

// 点积注意力函数
pub fn scaled_dot_product_attention(
    query: &Tensor,
    key: &Tensor,
    value: &Tensor,
) -> Result<Tensor, String> {
    // 简化版实现，假设已经在前一个练习中完成
    // 在实际实现中，这里应该有完整的点积注意力计算
    
    // 检查维度
    if query.shape().len() != 3 || key.shape().len() != 3 || value.shape().len() != 3 {
        return Err("注意力计算需要3D张量".to_string());
    }
    
    // 简化的实现，直接返回value
    // 在实际实现中，这里应该计算注意力权重并与value加权求和
    Ok(value.clone())
}

// 多头注意力结构体
pub struct MultiHeadAttention {
    num_heads: usize,
    d_model: usize,
    d_head: usize,
    w_query: Tensor, // 查询权重矩阵
    w_key: Tensor,   // 键权重矩阵
    w_value: Tensor, // 值权重矩阵
    w_output: Tensor, // 输出权重矩阵
}

impl MultiHeadAttention {
    // 创建一个新的多头注意力实例
    pub fn new(num_heads: usize, d_model: usize) -> Result<Self, String> {
        if d_model % num_heads != 0 {
            return Err(format!(
                "模型维度 {} 不能被头数 {} 整除",
                d_model, num_heads
            ));
        }

        let d_head = d_model / num_heads;

        // 初始化权重矩阵
        let w_query = Tensor::random(vec![d_model, d_model]);
        let w_key = Tensor::random(vec![d_model, d_model]);
        let w_value = Tensor::random(vec![d_model, d_model]);
        let w_output = Tensor::random(vec![d_model, d_model]);

        Ok(MultiHeadAttention {
            num_heads,
            d_model,
            d_head,
            w_query,
            w_key,
            w_value,
            w_output,
        })
    }

    // TODO: 实现线性投影函数
    // 将输入张量通过权重矩阵进行线性变换
    fn project(&self, x: &Tensor, weight: &Tensor) -> Result<Tensor, String> {
        // 在这里实现函数
        // 提示：使用矩阵乘法实现线性变换
        unimplemented!("实现线性投影函数");
    }

    // TODO: 实现多头注意力的前向传播
    pub fn forward(&self, query: &Tensor, key: &Tensor, value: &Tensor) -> Result<Tensor, String> {
        // 在这里实现函数
        // 步骤：
        // 1. 对Q、K、V进行线性投影
        // 2. 将投影后的张量分割成多个头
        // 3. 对每个头分别计算注意力
        // 4. 合并多头的输出
        // 5. 通过输出权重矩阵进行最终的线性变换
        unimplemented!("实现多头注意力的前向传播");
    }
}

// 不要修改测试代码
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_project() {
        // 创建一个简单的多头注意力实例
        let mha = MultiHeadAttention::new(2, 4).unwrap();

        // 创建一个输入张量 [1, 1, 4]
        let mut x = Tensor::zeros(vec![1, 1, 4]);
        for i in 0..4 {
            x.set(&[0, 0, i], i as f32 + 1.0).unwrap(); // [1, 2, 3, 4]
        }

        // 创建一个简单的权重矩阵 [4, 4]
        let mut w = Tensor::zeros(vec![4, 4]);
        for i in 0..4 {
            for j in 0..4 {
                if i == j {
                    w.set(&[i, j], 1.0).unwrap(); // 单位矩阵
                }
            }
        }

        // 测试投影函数
        let projected = mha.project(&x, &w).unwrap();

        // 验证形状
        assert_eq!(projected.shape(), &vec![1, 1, 4]);

        // 由于权重是单位矩阵，投影后的值应该与输入相同
        for i in 0..4 {
            assert_eq!(projected.get(&[0, 0, i]).unwrap(), i as f32 + 1.0);
        }
    }

    #[test]
    fn test_forward() {
        // 创建一个简单的多头注意力实例
        let mha = MultiHeadAttention::new(2, 4).unwrap();

        // 创建输入张量 [1, 2, 4]
        let mut query = Tensor::zeros(vec![1, 2, 4]);
        let mut key = Tensor::zeros(vec![1, 2, 4]);
        let mut value = Tensor::zeros(vec![1, 2, 4]);

        // 简单初始化
        for i in 0..2 {
            for j in 0..4 {
                query.set(&[0, i, j], 0.1).unwrap();
                key.set(&[0, i, j], 0.1).unwrap();
                value.set(&[0, i, j], 0.1).unwrap();
            }
        }

        // 测试前向传播
        let output = mha.forward(&query, &key, &value).unwrap();

        // 验证输出形状
        assert_eq!(output.shape(), &vec![1, 2, 4]);
    }

    #[test]
    fn test_split_and_concat_heads() {
        // 创建一个输入张量 [1, 2, 4]
        let mut x = Tensor::zeros(vec![1, 2, 4]);
        for b in 0..1 {
            for s in 0..2 {
                for i in 0..4 {
                    x.set(&[b, s, i], (b * 8 + s * 4 + i) as f32).unwrap();
                }
            }
        }

        // 分割成2个头
        let heads = x.split_heads(2).unwrap();

        // 验证头数
        assert_eq!(heads.len(), 2);

        // 验证每个头的形状
        for head in &heads {
            assert_eq!(head.shape(), &vec![1, 2, 2]);
        }

        // 合并头
        let merged = Tensor::concat_heads(&heads).unwrap();

        // 验证合并后的形状
        assert_eq!(merged.shape(), &vec![1, 2, 4]);

        // 验证合并后的值与原始值相同
        for b in 0..1 {
            for s in 0..2 {
                for i in 0..4 {
                    assert_eq!(
                        merged.get(&[b, s, i]).unwrap(),
                        x.get(&[b, s, i]).unwrap()
                    );
                }
            }
        }
    }
}