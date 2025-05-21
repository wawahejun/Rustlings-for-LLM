// 向量操作基础 - 练习1
//
// 在大型语言模型中，向量操作是最基础的计算单元。
// 本练习将帮助你学习如何在Rust中创建、访问和修改向量。
//
// 任务：
// 1. 实现一个函数，创建指定长度的零向量
// 2. 实现一个函数，计算两个向量的元素级加法
// 3. 实现一个函数，计算向量的L2范数（欧几里得范数）

// 不要修改这个结构体
#[derive(Debug, Clone, PartialEq)]
pub struct Vector {
    data: Vec<f32>,
}

impl Vector {
    // TODO: 实现一个创建指定长度的零向量的函数
    pub fn zeros(size: usize) -> Self {
        // 在这里实现函数
        unimplemented!("创建一个长度为 {} 的零向量", size);
    }

    // 获取向量长度
    pub fn len(&self) -> usize {
        self.data.len()
    }

    // 获取向量中指定索引的元素
    pub fn get(&self, index: usize) -> Option<f32> {
        if index < self.data.len() {
            Some(self.data[index])
        } else {
            None
        }
    }

    // 设置向量中指定索引的元素
    pub fn set(&mut self, index: usize, value: f32) -> Result<(), String> {
        if index < self.data.len() {
            self.data[index] = value;
            Ok(())
        } else {
            Err(format!("索引 {} 超出向量长度 {}", index, self.data.len()))
        }
    }

    // TODO: 实现两个向量的元素级加法
    pub fn add(&self, other: &Vector) -> Result<Vector, String> {
        // 在这里实现函数
        // 注意：两个向量长度必须相同，否则返回错误
        unimplemented!("实现两个向量的元素级加法");
    }

    // TODO: 实现计算向量的L2范数（欧几里得范数）
    // L2范数定义为：sqrt(sum(x_i^2))
    pub fn l2_norm(&self) -> f32 {
        // 在这里实现函数
        unimplemented!("计算向量的L2范数");
    }
}

// 不要修改测试代码
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zeros() {
        let vec = Vector::zeros(5);
        assert_eq!(vec.len(), 5);
        for i in 0..5 {
            assert_eq!(vec.get(i), Some(0.0));
        }
    }

    #[test]
    fn test_add() {
        let mut vec1 = Vector::zeros(3);
        vec1.set(0, 1.0).unwrap();
        vec1.set(1, 2.0).unwrap();
        vec1.set(2, 3.0).unwrap();

        let mut vec2 = Vector::zeros(3);
        vec2.set(0, 4.0).unwrap();
        vec2.set(1, 5.0).unwrap();
        vec2.set(2, 6.0).unwrap();

        let result = vec1.add(&vec2).unwrap();
        assert_eq!(result.get(0), Some(5.0));
        assert_eq!(result.get(1), Some(7.0));
        assert_eq!(result.get(2), Some(9.0));

        // 测试长度不同的情况
        let vec3 = Vector::zeros(2);
        assert!(vec1.add(&vec3).is_err());
    }

    #[test]
    fn test_l2_norm() {
        let mut vec = Vector::zeros(3);
        vec.set(0, 3.0).unwrap();
        vec.set(1, 4.0).unwrap();
        vec.set(2, 0.0).unwrap();

        // 3^2 + 4^2 + 0^2 = 9 + 16 = 25, sqrt(25) = 5
        assert_eq!(vec.l2_norm(), 5.0);

        let mut vec2 = Vector::zeros(2);
        vec2.set(0, 1.0).unwrap();
        vec2.set(1, 1.0).unwrap();

        // 1^2 + 1^2 = 2, sqrt(2) ≈ 1.4142
        assert!((vec2.l2_norm() - 1.4142).abs() < 0.001);
    }
}