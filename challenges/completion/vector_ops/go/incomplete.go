package main

import (
	"fmt"
)

type Vec2 struct {
	X, Y float64
}

func NewVec2(x, y float64) Vec2 {
	return Vec2{X: x, Y: y}
}

// Addition
func (v Vec2) Add(other Vec2) Vec2 {
	return Vec2{X: v.X + other.X, Y: v.Y + other.Y}
}

// Subtraction
func (v Vec2) Sub(other Vec2) Vec2 {
	return Vec2{X: v.X - other.X, Y: v.Y - other.Y}
}

// TODO: Componentwise multiplication
// func (v Vec2) Mul(other Vec2) Vec2 { ... }

// TODO: Componentwise division
// func (v Vec2) Div(other Vec2) Vec2 { ... }

