# README

## 开发文档：

由于模型可以适配多个游戏，所以不同游戏的一些逻辑放在games路径下，在根目录下的`game.py`是games路径下的某个游戏文件的`game.py`。
宝石在`games/splendor`的`splendor.py`中定义。但是与之绑定的还有`raw_config.py`。里面包含了更多的配置和常量信息（懒得分离了）。
复制到根目录下运行最好改名为`game.py`和`config.py`
在/py_splendor路径下运行`sh ./begin.sh`，它进行了复制。这样每次更新文件可只在games路径下更新。

或者保证game.py和config.py的最新后，直接运行main.py

## Splendor-ML

### 仓库结构

### RustCLI-Splendor

选择原因：
* 文档方便，开发效率高
* 类型系统完善
* 如果之后还有兴致的话，适合用此引入联机（内存安全、和Server-Client的性能+文档舒适度……）

因为splendor是终端回合制的，所以帧同步可以定为回合同步，这时候可以引入更多的约束，因为我们不需要常常去检查。

### JuCLI-Splendor

方便编写。专门优化代码后性能比较好。
**如果时间重组**，我们希望用SplendorML的策略特征来一个有限状态机

### QLearning 
一个QLearning机器学习方法的展示，这是我们进一步开发的基础

### MCTS&&RestNN Simplified-Splendor AI




* 为什么不使用MiniMax?
状态空间太大
演示：
* 为什么不选用Alpha-Beta剪枝？
* 激活函数：sigmoid
* 损失函数: softmax交叉熵
* 选用交叉熵作为损失函数:相比于二次loss的:
$$
\propto \frac{(y-a)^2}{2}
$$
交叉熵的Loss函数:
$$
L = \frac{-1}{n}\Sigma{yIna+(1-y)In(1-a)}(n表示样本总数)\\
\text{重新计算的参数w的梯度}:
\frac{\partial L}{\partial w} = \frac{1}{n}\Sigma_x x_j(\sigma(z) - y)
$$
后者误差越大，梯度越大在一定训练次数下loss的降低会更好.
`tf.nn`有`softmax`, `sparse`, `sigmoid`, `weighted`交叉熵。
$$
f(y_i|x)=\frac{e^{y_i}}{\Sigma_c e^{y_c}}
$$

$$
sigmoid = \frac{1}{11+e^{-x}}
$$

于是选用`tf.nn.softmax_cross_entropy_with_logits`

我们正好使用了sogmoid，那么对于神经元组我们使用softmax就自然了。s
方便构建模型和训练和做出demo。






### ResNN




### 问题和缺陷
还没有很方便的API可以将训练出来的模型与其他用户相通…缺乏AI开发的经验。
博弈论角度，为什么这种蒙卡树搜索适合打明牌游戏
## Installation

<!-- TODO -->



## More...


