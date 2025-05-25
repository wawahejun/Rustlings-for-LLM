// 张量计算 - 练习1
//
// 张量是大型语言模型的核心数据结构，它是向量和矩阵的高维推广。
// 本练习将帮助你学习如何在Rust中创建和操作张量。
//
// 任务：
// 1. 实现张量的创建函数
// 2. 实现张量元素的访问和修改
// 3. 实现张量形状的操作（reshape、transpose）

use std::fmt;

// 张量结构体，使用Vec<f32>存储数据，shape表示张量的维度
#[derive(Clone, PartialEq)]
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

    // 创建指定形状的全1张量
    pub fn ones(shape: Vec<usize>) -> Self {
        let size = shape.iter().product();
        Tensor {
            data: vec![1.0; size],
            shape,
        }
    }

    // 获取张量在指定索引处的值
    pub fn get(&self, indices: &[usize]) -> Result<f32, String> {
        if indices.len() != self.shape.len() {
            return Err(format!("索引维度({})与张量维度({})不匹配", indices.len(), self.shape.len()));
        }
        
        for (i, &idx) in indices.iter().enumerate() {
            if idx >= self.shape[i] {
                return Err(format!("索引越界: 维度{}的索引{}超出范围[0,{})", i, idx, self.shape[i]));
            }
        }
        
        let mut flat_idx = 0;
        let mut stride = 1;
        
        for i in (0..indices.len()).rev() {
            flat_idx += indices[i] * stride;
            stride *= self.shape[i];
        }
        
        Ok(self.data[flat_idx])
    }

    // 设置张量在指定索引处的值
    pub fn set(&mut self, indices: &[usize], value: f32) -> Result<(), String> {
        if indices.len() != self.shape.len() {
            return Err(format!("索引维度({})与张量维度({})不匹配", indices.len(), self.shape.len()));
        }
        
        for (i, &idx) in indices.iter().enumerate() {
            if idx >= self.shape[i] {
                return Err(format!("索引越界: 维度{}的索引{}超出范围[0,{})", i, idx, self.shape[i]));
            }
        }
        
        let mut flat_idx = 0;
        let mut stride = 1;
        
        for i in (0..indices.len()).rev() {
            flat_idx += indices[i] * stride;
            stride *= self.shape[i];
        }
        
        self.data[flat_idx] = value;
        Ok(())
    }

    // 张量的reshape操作
    pub fn reshape(&self, new_shape: Vec<usize>) -> Result<Tensor, String> {
        let new_size: usize = new_shape.iter().product();
        
        if new_size != self.data.len() {
            return Err(format!("新形状的总元素数({})与原形状的总元素数({})不匹配", new_size, self.data.len()));
        }
        
        Ok(Tensor {
            data: self.data.clone(),
            shape: new_shape,
        })
    }

    // 2D张量的转置操作
    pub fn transpose_2d(&self) -> Result<Tensor, String> {
        if self.shape.len() != 2 {
            return Err("转置操作仅支持2D张量".to_string());
        }
        
        let rows = self.shape[0];
        let cols = self.shape[1];
        let mut new_data = vec![0.0; rows * cols];
        
        for i in 0..rows {
            for j in 0..cols {
                let old_idx = i * cols + j;
                let new_idx = j * rows + i;
                new_data[new_idx] = self.data[old_idx];
            }
        }
        
        Ok(Tensor {
            data: new_data,
            shape: vec![cols, rows],
        })
    }
}

// 为Tensor实现Debug trait，方便打印和调试
impl fmt::Debug for Tensor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Tensor(shape={:?}, data={:?})", self.shape, self.data)
    }
}

// 不要修改测试代码
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zeros() {
        let tensor = Tensor::zeros(vec![2, 3]);
        assert_eq!(tensor.shape(), &vec![2, 3]);
        assert_eq!(tensor.size(), 6);
        for i in 0..6 {
            assert_eq!(tensor.data[i], 0.0);
        }
    }

    #[test]
    fn test_ones() {
        let tensor = Tensor::ones(vec![2, 2]);
        assert_eq!(tensor.shape(), &vec![2, 2]);
        assert_eq!(tensor.size(), 4);
        for i in 0..4 {
            assert_eq!(tensor.data[i], 1.0);
        }
    }

    #[test]
    fn test_get_set() {
        let mut tensor = Tensor::zeros(vec![2, 3]);

        // 设置值
        tensor.set(&[0, 1], 5.0).unwrap();
        tensor.set(&[1, 2], 7.0).unwrap();

        // 获取值
        assert_eq!(tensor.get(&[0, 1]).unwrap(), 5.0);
        assert_eq!(tensor.get(&[1, 2]).unwrap(), 7.0);
        assert_eq!(tensor.get(&[0, 0]).unwrap(), 0.0);

        // 测试索引越界
        assert!(tensor.get(&[2, 0]).is_err());
        assert!(tensor.set(&[0, 3], 10.0).is_err());
    }

    #[test]
    fn test_reshape() {
        let tensor = Tensor::ones(vec![2, 3]);
        let reshaped = tensor.reshape(vec![3, 2]).unwrap();
        assert_eq!(reshaped.shape(), &vec![3, 2]);
        assert_eq!(reshaped.size(), 6);

        // 测试元素数不匹配的情况
        assert!(tensor.reshape(vec![2, 2]).is_err());
    }

    #[test]
    fn test_transpose_2d() {
        let mut tensor = Tensor::zeros(vec![2, 3]);
        tensor.set(&[0, 0], 1.0).unwrap();
        tensor.set(&[0, 1], 2.0).unwrap();
        tensor.set(&[0, 2], 3.0).unwrap();
        tensor.set(&[1, 0], 4.0).unwrap();
        tensor.set(&[1, 1], 5.0).unwrap();
        tensor.set(&[1, 2], 6.0).unwrap();

        let transposed = tensor.transpose_2d().unwrap();
        assert_eq!(transposed.shape(), &vec![3, 2]);
        assert_eq!(transposed.get(&[0, 0]).unwrap(), 1.0);
        assert_eq!(transposed.get(&[1, 0]).unwrap(), 2.0);
        assert_eq!(transposed.get(&[2, 0]).unwrap(), 3.0);
        assert_eq!(transposed.get(&[0, 1]).unwrap(), 4.0);
        assert_eq!(transposed.get(&[1, 1]).unwrap(), 5.0);
        assert_eq!(transposed.get(&[2, 1]).unwrap(), 6.0);

        // 测试非2D张量
        let tensor_3d = Tensor::zeros(vec![2, 2, 2]);
        assert!(tensor_3d.transpose_2d().is_err());
    }
}