---
title: Cross Product
---

The **cross product** is an operation that takes two vectors in
$$ \mathbb{R}^3 $$

and returns another vector in $$ \mathbb{R}^3 $$

, written $\mathbf{a} \times \mathbf{b}$. Unlike the dot product, the result is
a vector, not a scalar. The cross product is only defined in three dimensions
(and in a generalized sense in seven dimensions; here we restrict to
$\mathbb{R}^3$).

## Geometric meaning

- **Direction:** $\mathbf{a} \times \mathbf{b}$ is perpendicular to both
  $\mathbf{a}$ and $\mathbf{b}$, following the right-hand rule: if you point
  your fingers along $\mathbf{a}$ and curl them toward $\mathbf{b}$, your thumb
  points in the direction of $\mathbf{a} \times \mathbf{b}$.
- **Magnitude:** $\|\mathbf{a} \times \mathbf{b}\| =
  \|\mathbf{a}\|\,\|\mathbf{b}\|\sin\theta$, where $\theta$ is the angle between
  $\mathbf{a}$ and $\mathbf{b}$. So the length equals the area of the
  parallelogram spanned by $\mathbf{a}$ and $\mathbf{b}$.

## Algebraic definition

For vectors

$$
\mathbf{a} =
\begin{pmatrix}
a_1\\
a_2\\
a_3
\end{pmatrix},
\quad
\mathbf{b} =
\begin{pmatrix}
b_1\\
b_2\\
b_3
\end{pmatrix},
$$

the cross product is

$$
\mathbf{a} \times \mathbf{b} =
\begin{pmatrix}
a_2 b_3 - a_3 b_2\\
a_3 b_1 - a_1 b_3\\
a_1 b_2 - a_2 b_1
\end{pmatrix}.
$$

This can be remembered using the determinant of a formal $3\times 3$ matrix:

$$

\mathbf{a} \times \mathbf{b} = \begin{vmatrix} \mathbf{e}_1 & \mathbf{e}_2 &
\mathbf{e}_3\\ a_1 & a_2 & a_3\\ b_1 & b_2 & b_3 \end{vmatrix}

\mathbf{e}_1(a_2 b_3 - a_3 b_2)

- \mathbf{e}_2(a_1 b_3 - a_3 b_1)

* \mathbf{e}_3(a_1 b_2 - a_2 b_1),

$$

where $\mathbf{e}_1, \mathbf{e}_2, \mathbf{e}_3$ are the standard unit vectors
in $\mathbb{R}^3$.

## Rules of calculation (with examples in LaTeX)

Let $\mathbf{a}, \mathbf{b}, \mathbf{c} \in \mathbb{R}^3$ and $\lambda \in
\mathbb{R}$.

---

**1. Anticommutativity**

Swapping the order flips the sign:

$$
\mathbf{a} \times \mathbf{b} = -\bigl(\mathbf{b} \times \mathbf{a}\bigr).
$$

Example:

$$
\begin{pmatrix} 1\\ 0\\ 0 \end{pmatrix} \times \begin{pmatrix} 0\\ 1\\ 0 \end{pmatrix}
= \begin{pmatrix} 0\\ 0\\ 1 \end{pmatrix},
\quad
\begin{pmatrix} 0\\ 1\\ 0 \end{pmatrix} \times \begin{pmatrix} 1\\ 0\\ 0 \end{pmatrix}
= \begin{pmatrix} 0\\ 0\\ -1 \end{pmatrix}.
$$

---

**2. Distributivity over addition**

$$
\mathbf{a} \times (\mathbf{b} + \mathbf{c})
= \mathbf{a} \times \mathbf{b} + \mathbf{a} \times \mathbf{c},
\qquad
(\mathbf{a} + \mathbf{b}) \times \mathbf{c}
= \mathbf{a} \times \mathbf{c} + \mathbf{b} \times \mathbf{c}.
$$

Example (second component of $\mathbf{a} \times (\mathbf{b}+\mathbf{c})$):

$$
\mathbf{a} = \begin{pmatrix} 1\\ 2\\ 0 \end{pmatrix},\;
\mathbf{b} = \begin{pmatrix} 0\\ 1\\ 1 \end{pmatrix},\;
\mathbf{c} = \begin{pmatrix} 1\\ 0\\ 1 \end{pmatrix}
\;\Rightarrow\;
\mathbf{b}+\mathbf{c} = \begin{pmatrix} 1\\ 1\\ 2 \end{pmatrix}.
$$

$$
\mathbf{a} \times \mathbf{b} = \begin{pmatrix} 2\\ -1\\ 1 \end{pmatrix},\quad
\mathbf{a} \times \mathbf{c} = \begin{pmatrix} 2\\ -1\\ -2 \end{pmatrix}
\;\Rightarrow\;
\mathbf{a} \times \mathbf{b} + \mathbf{a} \times \mathbf{c} = \begin{pmatrix} 4\\ -2\\ -1 \end{pmatrix}.
$$

$$
\mathbf{a} \times (\mathbf{b}+\mathbf{c}) = \begin{pmatrix} 2\cdot 2 - 0\cdot 1\\ 0\cdot 1 - 1\cdot 2\\ 1\cdot 1 - 2\cdot 1 \end{pmatrix} = \begin{pmatrix} 4\\ -2\\ -1 \end{pmatrix}.
$$

---

**3. Scalar multiplication (homogeneity)**

A scalar can be factored out of either slot:

$$
(\lambda \mathbf{a}) \times \mathbf{b}
= \mathbf{a} \times (\lambda \mathbf{b})
= \lambda (\mathbf{a} \times \mathbf{b}).
$$

Example: with $\mathbf{a} = \begin{pmatrix} 1\\ 0\\ 0 \end{pmatrix}$,
$\mathbf{b} = \begin{pmatrix} 0\\ 1\\ 0 \end{pmatrix}$, $\lambda = 3$,

$$
(3\mathbf{a}) \times \mathbf{b}
= \begin{pmatrix} 3\\ 0\\ 0 \end{pmatrix} \times \begin{pmatrix} 0\\ 1\\ 0 \end{pmatrix}
= \begin{pmatrix} 0\\ 0\\ 3 \end{pmatrix}
= 3 \begin{pmatrix} 0\\ 0\\ 1 \end{pmatrix}
= 3(\mathbf{a} \times \mathbf{b}).
$$

---

**4. Cross product with the zero vector**

$$
\mathbf{a} \times \mathbf{0} = \mathbf{0} \times \mathbf{a} = \mathbf{0}.
$$

---

**5. Parallel vectors**

$\mathbf{a}$ and $\mathbf{b}$ are parallel (or one is zero) if and only if

$$
\mathbf{a} \times \mathbf{b} = \mathbf{0}.
$$

Example: $\mathbf{a} = \begin{pmatrix} 2\\ 4\\ 6 \end{pmatrix}$, $\mathbf{b}
= \begin{pmatrix} 1\\ 2\\ 3 \end{pmatrix} = \tfrac{1}{2}\mathbf{a}$, so

$$
\mathbf{a} \times \mathbf{b}
= \begin{pmatrix} 4\cdot 3 - 6\cdot 2\\ 6\cdot 1 - 2\cdot 3\\ 2\cdot 2 - 4\cdot 1 \end{pmatrix}
= \begin{pmatrix} 0\\ 0\\ 0 \end{pmatrix}.
$$

---

**6. Self-cross product**

$$
\mathbf{a} \times \mathbf{a} = \mathbf{0}.
$$

(Special case of the parallel-vectors rule.)

---

**7. Jacobi identity**

$$

\mathbf{a} \times (\mathbf{b} \times \mathbf{c})

- \mathbf{b} \times (\mathbf{c} \times \mathbf{a})
- \mathbf{c} \times (\mathbf{a} \times \mathbf{b}) = \mathbf{0}.

$$

---

**8. Relation to dot product (vector triple product expansion)**

$$
\mathbf{a} \times (\mathbf{b} \times \mathbf{c})
= (\mathbf{a} \cdot \mathbf{c})\mathbf{b} - (\mathbf{a} \cdot \mathbf{b})\mathbf{c}.
$$

Example: $\mathbf{a} = \mathbf{e}_1$, $\mathbf{b} = \mathbf{e}_2$,
$\mathbf{c} = \mathbf{e}_3$:

$$
\mathbf{a} \cdot \mathbf{c} = 0,\quad \mathbf{a} \cdot \mathbf{b} = 0
\;\Rightarrow\;
\mathbf{a} \times (\mathbf{b} \times \mathbf{c}) = 0\cdot \mathbf{b} - 0\cdot \mathbf{c} = \mathbf{0}.
$$

$$
\mathbf{b} \times \mathbf{c} = \mathbf{e}_1
\;\Rightarrow\;
\mathbf{e}_1 \times \mathbf{e}_1 = \mathbf{0}.
$$

---

**9. Magnitude and angle**

$$
\|\mathbf{a} \times \mathbf{b}\|^2
= \|\mathbf{a}\|^2 \|\mathbf{b}\|^2 - (\mathbf{a} \cdot \mathbf{b})^2.
$$

Equivalently, $\|\mathbf{a} \times \mathbf{b}\| =
\|\mathbf{a}\|\,\|\mathbf{b}\|\sin\theta$.

---

**10. Relation to the dot product (scalar triple product)**

$$
\mathbf{a} \cdot (\mathbf{b} \times \mathbf{c})
= \mathbf{b} \cdot (\mathbf{c} \times \mathbf{a})
= \mathbf{c} \cdot (\mathbf{a} \times \mathbf{b}).
$$

This value is the (signed) volume of the parallelepiped spanned by $\mathbf{a},
\mathbf{b}, \mathbf{c}$. Example:

$$
\mathbf{a} = \begin{pmatrix} 1\\ 0\\ 0 \end{pmatrix},\;
\mathbf{b} = \begin{pmatrix} 0\\ 1\\ 0 \end{pmatrix},\;
\mathbf{c} = \begin{pmatrix} 0\\ 0\\ 1 \end{pmatrix}
\;\Rightarrow\;
\mathbf{b} \times \mathbf{c} = \begin{pmatrix} 1\\ 0\\ 0 \end{pmatrix},\quad
\mathbf{a} \cdot (\mathbf{b} \times \mathbf{c}) = 1.
$$

## Worked example

Compute $\mathbf{u} \times \mathbf{v}$ for

$$
\mathbf{u} = \begin{pmatrix} 2\\ -1\\ 3 \end{pmatrix},\qquad
\mathbf{v} = \begin{pmatrix} 1\\ 4\\ -2 \end{pmatrix}.
$$

$$
\mathbf{u} \times \mathbf{v}
= \begin{pmatrix}
(-1)(-2) - (3)(4)\\
(3)(1) - (2)(-2)\\
(2)(4) - (-1)(1)
\end{pmatrix}
= \begin{pmatrix}
2 - 12\\
3 + 4\\
8 + 1
\end{pmatrix}
= \begin{pmatrix} -10\\ 7\\ 9 \end{pmatrix}.
$$

Check: $\mathbf{u} \cdot (\mathbf{u} \times \mathbf{v}) = 2(-10) + (-1)(7) +
3(9) = -20 - 7 + 27 = 0$, and $\mathbf{v} \cdot (\mathbf{u} \times \mathbf{v})
= 1(-10) + 4(7) + (-2)(9) = -10 + 28 - 18 = 0$, so the result is perpendicular
to both $\mathbf{u}$ and $\mathbf{v}$.
