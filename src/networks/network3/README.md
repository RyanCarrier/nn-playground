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

$o^l_j=b^l_j+\sum_{i=0}^{N^l}{a_i^{l-1}w^l_{ij}}$

<!-- $\gamma(o^l_i)=\begin{cases} -->
<!-- o^l_i \gt0 & o^l_i\\ -->
<!-- o^l_i \le0 & 0 -->
<!-- \end{cases}$, we use this because o.min(0.0) is easy, and not very computationally heavy -->

$\gamma(o^l_i)=\
\frac{1}{1+e^{-o^l_i}}
$
, honestly the idea of using o.min(0.0) felt better, but sigmoid fn is used more often

$A^l=(a^l_{j})=\gamma(o^l_j)$, change our output into a better range of values

$y(x)=A^L(A^{L-1}(\cdots A^2(A^1(x))))$

$E=\frac{1}{2}(x-y)^2$
, we use this cause square error is nice, but the 1/2 makes derivate simpler (later)

### What we want

We want to know how much does the error change (and which way), based on changes in the each weight and bias. That way we can make adjustments to weights and biases that we can be pretty confident will reduce the error rate.

$\frac{\partial{E}}{\partial{w^l_{ij}}}$: error change wrt a weight

$\frac{\partial{E}}{\partial{b^l_{j}}}$: error wrt a bias

$E=\frac{1}{2}(t-y)^2$


Don't have $E$ wrt $w^l_{ij}$, so need to chain rule. Chain rule is simply chaining relationships together to acheive the desired derivative;

Starting with the final (output layer)

#### Output layer

$\frac{\partial{E}}{\partial{y}}\frac{\partial{y}}{\partial{O^L}}\frac{\partial{O^L}}{\partial{w^L_{ij}}}=\frac{\partial{E}}{\partial{w^L_{ij}}}$

And note that $y\equiv A^L$

We can represent all 
$
\frac{\partial{E}}{\partial{y}},
\frac{\partial{y}}{\partial{o^L_{ij}}}$ and $
\frac{\partial{o^L_{ij}}}{\partial{w^l_{ij}}}$ individually.

<hr>

$E=\frac{1}{2}(t-y)^2$

$\frac{\partial{E}}{\partial{y}}=y-t$

<hr>

$y(x)=A^L=\gamma (O^L)$


$
\frac{\partial{y}}{\partial{O^L}}=
\frac{\partial{A^L}}{\partial{O^L}}=
\gamma '(O^L)=\gamma{(O^L)}(1-\gamma (O^L))$
, the derivative of the sigmoid fn

<hr>

$O^L=B^L+\sum{A^{L-1}W^L}$

$O^L=(o^L_m)=(b^L_m+\sum_{k=0}^{N^L}{a^{L-1}_k w^L_{km}})$

The only piece of the set which would not derive to 0, would be the term with $m=j$, so;

$O^L\equiv o^L_j=b^L_j+\sum_{k=0}^{N^L}{a^{L-1}_k w^L_{kj}}$

$\frac{\partial{O^L}}{\partial{w^L_{ij}}}={a_i^{L-1}}$

<hr>

$
\frac{\partial{E}}{\partial{w^L_{ij}}}
=\
\frac{\partial{E}}{\partial{y}}
\frac{\partial{y}}{\partial{O^L}}
\frac{\partial{O^L}}{\partial{w^L_{ij}}}=\
\gamma'(o^l_j)(y_j-t_j)a_i^{L-1}\
$

#### Intermediate layers

$
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

<hr>

$
\frac{\partial{E}}{\partial{A^{l+1}}}$
will be provided by the next (previous in regards to back propogation) layer.

$
\frac{\partial{A^{l+1}}}{\partial{O^{l+1}}}
=\gamma '(O^{l+1})\
$

$
\frac{\partial{O^{l+1}}}{\partial{A^{l}}}
=\
\sum^{N^{l+1}}_{i=0} W^{l+1}_i
$

$
\frac{\partial{A^{l}}}{\partial{w^{l}_{ij}}}
=\
\frac{\partial{A^{l}}}{\partial{O^{l}}}
\frac{\partial{O^{l}}}{\partial{w^{l}_{ij}}}
=\gamma '(o^{l}_j)a^{l-1}_{i}\
$

We will need activation gradient for next layer anyway;

$
\frac{\partial{E}}{\partial{A^{l}}}\
=\
\frac{\partial{E}}{\partial{A^{l+1}}}
\gamma '(O^{l+1})\
\sum^{N^{l+1}}_{i=0} W^{l+1}_i\
$

$
\frac{\partial{E}}{\partial{w^{l}_{ij}}}
=\
\frac{\partial{E}}{\partial{A^{l}}}
\frac{\partial{A^{l}}}{\partial{w^{l}_{ij}}}
=\
\frac{\partial{E}}{\partial{A^{l}}}\
\left(
\gamma '(O^{l})a^{l-1}_{i}\
\right)
$


### Bias

$
\frac{\partial{O^{l}}}{\partial{B^{l}}}
=\frac{\partial{\left(B^l+\sum{A^{l-1} W^l}\right)}}{\partial{B^l}}
=1
$

$
\frac{\partial{A^{l}}}{\partial{B^{l}}}
=\
\frac{\partial{A^{l}}}{\partial{O^{l}}}
\frac{\partial{O^{l}}}{\partial{B^{l}}}
=\frac{\partial{A^{l}}}{\partial{O^{l}}}
=\gamma ' (O^l)$

$
\frac{\partial{E^{l}}}{\partial{B^{l}}}
=\
\frac{\partial{E}}{\partial{A^{l}}}
\frac{\partial{A^{l}}}{\partial{B^{l}}}
=\frac{\partial{E}}{\partial{A^{l}}}
\gamma ' (O^l)
$
 
### Summary

$
\frac{\partial{E}}{\partial{A^{l}}}
=\
\frac{\partial{E}}{\partial{A^{l+1}}}
\frac{\partial{A^{l+1}}}{\partial{O^{l+1}}}
\frac{\partial{O^{l+1}}}{\partial{A^{l}}}
=\frac{\partial{E}}{\partial{A^{l+1}}}
\gamma '(O^{l+1})\
\sum^{N^{l+1}}_{i=0} W^{l+1}_i\
$


$
\frac{\partial{E}}{\partial{w^{l}_{ij}}}
=\
\frac{\partial{E}}{\partial{A^{l}}}
\frac{\partial{A^{l}}}{\partial{w^{l}_{ij}}}
=\frac{\partial{E}}{\partial{A^{l}}}
\gamma '(O^{l})a^{l-1}_{i}\
$

$
\frac{\partial{E}}{\partial{b^{l}_{j}}}
=\
\frac{\partial{E}}{\partial{A^{l}}}
\frac{\partial{A^{l}}}{\partial{b^{l}_{j}}}
=\frac{\partial{E}}{\partial{A^{l}}}
\gamma ' (O^l)
$


