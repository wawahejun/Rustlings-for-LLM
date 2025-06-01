// 在深度学习中，梯度计算和反向传播是训练模型的关键。
// 在这个练习中，我们将实现简单的向量梯度计算。

fn vector_add(v1: &[f32], v2: &[f32]) -> Vec<f32> {
    assert_eq!(v1.len(), v2.len(), "向量长度必须相同");
    v1.iter()
        .zip(v2.iter())
        .map(|(&a, &b)| a + b)
        .collect()
}

// TODO: 实现向量乘以标量函数
// 将向量的每个元素乘以一个标量
fn vector_scale(v: &[f32], scale: f32) -> Vec<f32> {
    v.iter()
        .map(|&x| x * scale)
        .collect()
}

// TODO: 实现ReLU激活函数的前向传播
// f(x) = max(0, x)
fn relu_forward(x: &[f32]) -> Vec<f32> {
    x.iter()
        .map(|&x| x.max(0.0))
        .collect()
}

// TODO: 实现ReLU激活函数的反向传播
// 给定上游梯度和前向传播的输入，计算ReLU的梯度
fn relu_backward(upstream_grad: &[f32], x: &[f32]) -> Vec<f32> {
    assert_eq!(upstream_grad.len(), x.len(), "向量长度必须相同");
    upstream_grad.iter()
        .zip(x.iter())
        .map(|(&grad, &input)| if input > 0.0 { grad } else { 0.0 })
        .collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f32 = 1e-5;

    #[test]
    fn test_vector_add() {
        let v1 = vec![1.0, -2.0, 3.0];
        let v2 = vec![0.5, 1.0, -1.0];
        let result = vector_add(&v1, &v2);
        let expected = vec![1.5, -1.0, 2.0];
        
        assert_eq!(result.len(), expected.len(), "向量加法结果长度错误");
        for (a, b) in result.iter().zip(expected.iter()) {
            assert!((a - b).abs() < EPSILON, "向量加法计算错误");
        }
    }

    #[test]
    fn test_vector_scale() {
        let v = vec![1.0, -2.0, 3.0];
        let scale = 2.0;
        let result = vector_scale(&v, scale);
        let expected = vec![2.0, -4.0, 6.0];
        
        assert_eq!(result.len(), expected.len(), "向量缩放结果长度错误");
        for (a, b) in result.iter().zip(expected.iter()) {
            assert!((a - b).abs() < EPSILON, "向量缩放计算错误");
        }
    }

    #[test]
    fn test_relu_forward() {
        let x = vec![-1.0, 0.0, 2.0];
        let result = relu_forward(&x);
        let expected = vec![0.0, 0.0, 2.0];
        
        assert_eq!(result.len(), expected.len(), "ReLU前向传播结果长度错误");
        for (a, b) in result.iter().zip(expected.iter()) {
            assert!((a - b).abs() < EPSILON, "ReLU前向传播计算错误");
        }
    }

    #[test]
    fn test_relu_backward() {
        let upstream_grad = vec![1.0, 1.0, 1.0];
        let x = vec![-1.0, 0.0, 2.0];
        let result = relu_backward(&upstream_grad, &x);
        let expected = vec![0.0, 0.0, 1.0]; // 对于x<0的位置，梯度为0
        
        assert_eq!(result.len(), expected.len(), "ReLU反向传播结果长度错误");
        for (a, b) in result.iter().zip(expected.iter()) {
            assert!((a - b).abs() < EPSILON, "ReLU反向传播计算错误");
        }
    }
}