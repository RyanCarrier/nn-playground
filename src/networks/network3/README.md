# Backprop

Ok i got a good enough understanding i think to be able to lay the math down, dunno how to latex in markdown but that's another problem 

## quick mafs

### what we got

#### values

$L$: Layers\
$l$: layer ($0\le{l}\le{L}$)

$N^l$: nodes in layer ($N^0$ = input layer, $N^L$ = output layer)

$x$: Network input (of size $N^0$, same as $A^0$)\
$y$: Network output (of size $N^L$, same as $A^L$)\
$t$: Target network output (of size $N^L$)

$i$: source node, $0\le i\lt N^{l-1}$\
$j$: dest node/current node, $0\lt j\lt N^l$


$B^l=(b^l_j$): Bias for node $j$ in layer $l$\
$W^l=(w^l_{ij}$): Weight between node $i$ on layer $l-1$ to node $j$ on layer $l$\
$O^l=(o^l_{j}$): Output of node $j$ on layer $l$ \
$A^l=(a^l_{j}$): Activation of node $j$ on layer $l$ 

$\gamma(o^l_{ij})\rightarrow a^l_{ij}$: Activation function

$E(y,t)$: Error function, how 'heavy' or how much error is in this output compared to the target output

#### defs

$o^l_j=b^l_j+\displaystyle\sum_{i=0}^{N^l}{a_i^{l-1}w^l_{ij}}$

$\gamma(o^l_i)=\begin{cases}
o^l_i \gt0 & o^l_i\\
o^l_i \le0 & 0
\end{cases}$, we use this because o.min(0.0) is easy, and not very computationally heavy

$A^l=(a^l_{j})=\gamma(o^l_j)$, change our output into a better range of values

$y(x)=A^L(A^{L-1}(\cdots A^2(A^1(x))))$

$E=\frac{1}{2}(x-y)^2$
, we use this cause square error is nice, but the 1/2 makes derivate simpler (later)

### What we want

We want to know how much does the error change (and which way), based on changes in the each weight and bias. That way we can make adjustments to weights and biases that we can be pretty confident will reduce the error rate.

$\displaystyle\frac{\partial{E}}{\partial{w^l_{ij}}}$: error change wrt a weight

$\displaystyle\frac{\partial{E}}{\partial{b^l_{j}}}$: error wrt a bias

$E=\frac{1}{2}(t-y)^2$


Don't have $E$ wrt $w^l_{ij}$, so need to chain rule. Chain rule is simply chaining relationships together to acheive the desired derivative;

Starting with the final (output layer)

#### Output layer

$
\displaystyle
\frac{\partial{E}}{\partial{y}}
\frac{\partial{y}}{\partial{O^L}}
\frac{\partial{O^L}}{\partial{w^L_{ij}}}
=\
\frac{\partial{E}}{\cancel{\partial{y}}}
\frac{\cancel{\partial{y}}}{\cancel{\partial{O^L}}}
\frac{\cancel{\partial{O^L}}}{\partial{w^L_{ij}}}
=\
\frac{\partial{E}}{\partial{w^L_{ij}}}
$

And note that $y\equiv A^L$

We can represent all 
$\displaystyle
\frac{\partial{E}}{\partial{y}},
\frac{\partial{y}}{\partial{o^L_{ij}}}$ and $\displaystyle
\frac{\partial{o^L_{ij}}}{\partial{w^l_{ij}}}$ individually.

<hr>

$E=\frac{1}{2}(t-y)^2$

$\displaystyle\frac{\partial{E}}{\partial{y}}=y-t$

<hr>

$y(x)=A^L=\gamma (O^L)$

$\displaystyle\frac{\partial{y}}{\partial{O^L}}$ is not cts as $\gamma$ is not cts and so ;

$
\displaystyle\frac{\partial{y}}{\partial{O^L}}=
\begin{cases}
o^l_i \gt0 & 1\\
o^l_i = 0 & undefined...(0)\\
o^l_i \lt0 & 0
\end{cases}$

<hr>

$O^L=B^L+\displaystyle\sum{A^{L-1}W^L}$

$O^L=(o^L_m)=(b^L_m+\displaystyle\sum_{k=0}^{N^L}{a^{L-1}_k w^L_{km}})$

The only piece of the set which would not derive to 0, would be the term with $m=j$, so;

$O^L\equiv o^L_j=b^L_j+\displaystyle\sum_{k=0}^{N^L}{a^{L-1}_k w^L_{kj}}$

$\displaystyle\frac{\partial{O^L}}{\partial{w^L_{ij}}}={a_i^{L-1}}$

<hr>

$
\displaystyle
\frac{\partial{E}}{\partial{w^L_{ij}}}
=\
\frac{\partial{E}}{\partial{y}}
\frac{\partial{y}}{\partial{O^L}}
\frac{\partial{O^L}}{\partial{w^L_{ij}}}
=\begin{cases}
o^l_i \gt0 & (y-t)a_i^{L-1}\\
o^l_i \le0 & 0
\end{cases}\
$

#### Intermediate layers

$
\displaystyle
\frac{\partial{E}}{\partial{w^{L-1}_{ij}}}
=\
\frac{\partial{E}}{\partial{A^{L-1}}}
\frac{\partial{A^{L-1}}}{\partial{w^{L-1}_{ij}}}
=\left(\
\frac{\partial{E}}{\partial{A^{L}}}
\frac{\partial{A^L}}{\partial{O^L}}
\frac{\partial{O^L}}{\partial{A^{L-1}}}
\right)
\left(
\frac{\partial{A^{L-1}}}{\partial{O^{L-1}}}
\frac{\partial{O^{L-1}}}{\partial{w^{L-1}_{ij}}}
\right)
$


For the next layer...

$
\displaystyle
\frac{\partial{E}}{\partial{w^{L-2}_{ij}}}
=\
\frac{\partial{E}}{\partial{A^{L-2}}}
\frac{\partial{A^{L-2}}}{\partial{w^{L-2}_{ij}}}
=\left(\
\frac{\partial{E}}{\partial{A^{L-1}}}
\frac{\partial{A^{L-1}}}{\partial{O^{L-1}}}
\frac{\partial{O^{L-1}}}{\partial{A^{L-2}}}
\right)
\left(
\frac{\partial{A^{L-2}}}{\partial{O^{L-2}}}
\frac{\partial{O^{L-2}}}{\partial{w^{L-2}_{ij}}}
\right)
$

Note we can use some terms from the previous step...

So more generically;


$
\displaystyle
\frac{\partial{E}}{\partial{w^{l}_{ij}}}
=\
\frac{\partial{E}}{\partial{A^{l}}}
\frac{\partial{A^{l}}}{\partial{w^{l}_{ij}}}
=\left(\
\frac{\partial{E}}{\partial{A^{l+1}}}
\frac{\partial{A^{l+1}}}{\partial{O^{l+1}}}
\frac{\partial{O^{l+1}}}{\partial{A^{l}}}
\right)
\left(
\frac{\partial{A^{l}}}{\partial{O^{l}}}
\frac{\partial{O^{l}}}{\partial{w^{l}_{ij}}}
\right)
$

$
\displaystyle
\frac{\partial{E}}{\partial{A^{l+1}}}
$
will be provided by the next (previous in regards to back propogation) layer.

$
\displaystyle
\frac{\partial{A^{l+1}}}{\partial{O^{l+1}}}
=\begin{cases}
o^{l+1}_i \gt0 & 1\\
o^{l+1}_i \le0 & 0\
\end{cases}\
$

$
\displaystyle
\frac{\partial{O^{l+1}}}{\partial{A^{l}}}
=\
\sum^{N^{l+1}}_{i=0} W^{l+1}_i
$


