# Backprop

Ok i got a good enough understanding i think to be able to lay the math down, dunno how to latex in markdown but that's another problem 

## quick mafs

### what we got

#### values

$L$ = Layers

$l$ = layer ($0\le{l}\le{L}$)

$N^l$= nodes in layer ($N^0$ = input layer, $N^L$ = output layer)

$x$ = Network input (of size $N^0$)

$y$ = Network output (of size $N^L$)

$t$ = Target network output (of size $N^L$)

$i$ = source node, $0\le i\lt N^{l-1}$

$j$ = dest node/current node, $0\lt j\lt N^l$

$b^l_j$= Bias for node $j$ in layer $l$

$w^l_{ij}$= Weight between node $i$ on layer $l-1$ to node $j$ on layer $l$

$o^l_{j}$= Output of node $j$ on layer $l$ 

$a^l_{j}$= Activation of node $j$ on layer $l$ 

$B^l$= Biases for layer $l$, ($b_{ij}$)

$W^l$= Weights for layer $l$, ($w_{ij}$)

$O^l$= Outputs for layer $l$, ($o_{ij}$)

$A^l$= Activation for layer $l$, ($a_i$)

$\gamma(o^l_{ij})$ = Activation function (converts output of node to activation of node, $o^l_{ij}\rightarrow a^l_{ij}$)

$E(y,t)$ = Error function, how 'heavy' or how much error is in this output compared to the target output

#### defs

$o^l_j=b^l_j+\displaystyle\sum_{i=0}^{N^l}{a_i^{l-1}w^l_{ij}}$

$O^l=B^l+\sum A^{l-1}W^l$

This can look confusing, but this is just previous layer, multiplied by the weights to the next node (and it's bias).

$\gamma(o^l_i)=\begin{cases}
o^l_i \gt0 & o^l_i\\
o^l_i \le0 & 0
\end{cases}$, we use this because o.min(0.0) is easy, and not very computationally heavy

$a^l_{j}=\gamma(o^l_j)$, change our output into a better range of values

$A^l=\gamma(O^l)$, change our output into a better range of values

$y(x)=A^l(A^{l-1}(\cdots A^2(A^1(x))))$

$E=\frac{1}{2}(x-y)^2$
, we use this cause square error is nice, but the 1/2 makes derivate simpler (later)

### What we want

We want to know how much does the error change (and which way), based on changes in the each weight and bias

$\frac{\partial{E}}{\partial{w^l_{ij}}}$, error change wrt a weight

$\frac{\partial{E}}{\partial{b^l_{j}}}$, error wrt a bias

$E=\frac{1}{2}(t-y)^2$

$E$ is dependant on $y$, not $w_{ij}$

$y(x)=A^l$, or $\gamma (O^l)$

$y$ is dependant on $O^l$

$O^l=B^l+\sum A^{l-1}W^l$

$o^l_j=b^l_j+\displaystyle\sum_{i=0}^{N^l}{a_i^{l-1}w^l_{ij}}$

Which finally depends on $w^l_{ij}$

So using chain rule;

$
\displaystyle
\frac{\partial{E}}{\partial{w^l_{ij}}}=
\frac{\partial{E}}{\partial{y}}
\frac{\partial{y}}{\partial{o^l_{ij}}}
\frac{\partial{o^l_{ij}}}{\partial{w^l_{ij}}}
$

$E=\frac{1}{2}(t-y)^2$

$\displaystyle\frac{\partial{E}}{\partial{y}}=y-t$

$\displaystyle\frac{\partial{y}}{\partial{o^l_{ij}}}$ is tough as $\gamma$ is not cts and so;

$
\displaystyle\frac{\partial{y}}{\partial{o^l_{ij}}}=
\begin{cases}
o^l_i \gt0 & 1\\
o^l_i = 0 & undefined...(0)\\
o^l_i \lt0 & 0
\end{cases}$

$o^l_j=b^l_j+\displaystyle\sum_{i=0}^{N^l}{a_i^{l-1}w^l_{ij}}$

$\displaystyle\frac{\partial{o^l_{ij}}}{\partial{w^l_{ij}}}={a_i^{l-1}}$

So then all together;

$
\displaystyle
\frac{\partial{E}}{\partial{w^L_{ij}}}=
\frac{\partial{E}}{\partial{y}}
\frac{\partial{y}}{\partial{o^L_{ij}}}
\frac{\partial{o^L_{ij}}}{\partial{w^L_{ij}}}
=\begin{cases}
o^l_i \gt0 & (y-t){a_i^{l-1}}\\
o^l_i \le0 & 0
\end{cases}
$

Then for a weight not in the final layer...
Note that $y\equiv a^L$

$
\displaystyle
\frac{\partial{E}}{\partial{w^l_{ij}}}=
\frac{\partial{E}}{\partial{a^L_{j}}}
\frac{\partial a^L_{j}}{\partial{o^L_{ij}}}
\frac{\partial{o^{L}_{ij}}}{\partial a^{L-1}_{j}}
\frac{\partial a^{L-1}_{j}}{\partial{o^{L-1}_{ij}}}
\frac{\partial{o^{L-1}_{ij}}}{\partial{w^{L-1}_{ij}}}
$




### Output nodes

$``$

$``$
$``$
$``$
$``$
$``$


$``$
