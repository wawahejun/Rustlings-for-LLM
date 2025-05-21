# 注意力机制

注意力机制是Transformer架构的核心组件，也是大型语言模型能够处理长序列和捕获上下文关系的关键。

## 学习目标

- 理解自注意力（Self-Attention）的基本原理
- 实现点积注意力（Scaled Dot-Product Attention）
- 学习多头注意力（Multi-Head Attention）的实现方法

## 练习列表

- `attention1.rs`: 实现基本的点积注意力机制
- `attention2.rs`: 实现多头注意力机制
- `attention3.rs`: 实现带掩码的注意力机制（用于解码器自注意力）

完成这些练习后，你将理解大型语言模型中最核心的注意力计算过程，为理解整个Transformer架构打下基础。