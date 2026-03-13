---
title: Dot Product
---

The dot product is an operation that takes two vectors of the same dimension and
returns a single real number (a scalar), often written with a centered dot like
\(\mathbf{a} \cdot \mathbf{b}\).

## Algebraic definition

For vectors in $$ \mathbb{R}^n $$

$$
\mathbf{a} =
\begin{pmatrix}
a_1\\
a_2\\
\vdots\\
a_n
\end{pmatrix},
\quad
\mathbf{b} =
\begin{pmatrix}
b_1\\
b_2\\
\vdots\\
b_n
\end{pmatrix},
$$

their dot product is

$$

\mathbf{a} \cdot \mathbf{b} =

a_1 b_1 + a_2 b_2 + \dots + a_n b_n.

$$

Example in $$ \mathbb{R}^3 $$

$$

\begin{pmatrix} 1\\ 3\\ -5 \end{pmatrix} \cdot \begin{pmatrix} 4\\ -2\\ -1
\end{pmatrix} =

1\cdot 4 + 3\cdot(-2) + (-5)\cdot(-1) =

4 - 6 + 5 = 3.

$$

## Geometric definition

If $$ \mathbf{a}, \mathbf{b} \in \mathbb{R}^n $$

and $$ \theta $$

is the angle between them, then

$$ \mathbf{a} \cdot \mathbf{b} $$

$$ \|\mathbf{a}\|\;\|\mathbf{b}\|\cos\theta, $$

where $$ \|\mathbf{a}\| $$

is the Euclidean length (norm) of $$ \mathbf{a} $$

From this, you also get

$$
\mathbf{a} \cdot \mathbf{a} = \|\mathbf{a}\|^2,
\quad
\|\mathbf{a}\| = \sqrt{\mathbf{a} \cdot \mathbf{a}}.
$$

## Basic calculation rules

Let $$ \mathbf{a}, \mathbf{b}, \mathbf{c} \in \mathbb{R}^n $$

and $$ \lambda \in
\mathbb{R} $$

Then:

- Commutativity:

  $$
  \mathbf{a} \cdot \mathbf{b} = \mathbf{b} \cdot \mathbf{a}.
  $$

- Distributivity over addition:

  $$
  \mathbf{a} \cdot (\mathbf{b} + \mathbf{c}) =
  \mathbf{a} \cdot \mathbf{b} + \mathbf{a} \cdot \mathbf{c}.
  $$

- Homogeneity (scalar multiplication in one slot):

  $$
  (\lambda \mathbf{a}) \cdot \mathbf{b} =
   \lambda (\mathbf{a} \cdot \mathbf{b}), \quad \mathbf{a} \cdot (\lambda \mathbf{b}) =
  \lambda (\mathbf{a} \cdot \mathbf{b}).
  $$

- Positivity:

  $$
  \mathbf{a} \cdot \mathbf{a} \ge 0
  \quad \text{and} \quad
  \mathbf{a} \cdot \mathbf{a} = 0 \iff \mathbf{a} = \mathbf{0}.
  $$

## Worked examples

1. Simple 2D example

Let

$$
\mathbf{u} =
\begin{pmatrix}
2\\
-1
\end{pmatrix},
\quad
\mathbf{v} =
\begin{pmatrix}
3\\
4
\end{pmatrix}.
$$

Then

$$ \mathbf{u} \cdot \mathbf{v}
2\cdot 3 + (-1)\cdot 4 =
6 - 4 =
2.
$$

2. 4D example

Let

$$
\mathbf{x} =
\begin{pmatrix}
2\\
0\\
-3\\
1
\end{pmatrix},
\quad
\mathbf{y} =
\begin{pmatrix}
-1\\
3\\
1\\
2
\end{pmatrix}.
$$

Then

$$
\mathbf{x} \cdot \mathbf{y} =
2(-1) + 0(3) + (-3)(1) + 1(2) =
-2 + 0 - 3 + 2 =
-3.
$$

3. Using the geometric form to find an angle

Let

$$
\mathbf{a} =
\begin{pmatrix}
1\\
2
\end{pmatrix},
\quad
\mathbf{b} =
\begin{pmatrix}
2\\
1
\end{pmatrix}.
$$

Compute

$$

\mathbf{a} \cdot \mathbf{b} = 1\cdot 2 + 2\cdot 1 4,

$$

$$

\|\mathbf{a}\| =

\sqrt{1^2 + 2^2} =

\sqrt{5}, \quad \|\mathbf{b}\| =

\sqrt{2^2 + 1^2} =

\sqrt{5}.

$$

So

$$

\cos\theta =

\frac{\mathbf{a} \cdot \mathbf{b}}{\|\mathbf{a}\|\,\|\mathbf{b}\|} =

\frac{4}{\sqrt{5}\sqrt{5}} =

\frac{4}{5},

$$

and hence

$$
\theta = \arccos\!\left(\frac{4}{5}\right).
$$
