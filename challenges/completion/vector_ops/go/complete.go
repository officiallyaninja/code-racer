package vec2

type Vec2 struct {
	X, Y float64
}

func New(x, y float64) Vec2 {
	return Vec2{X: x, Y: y}
}

// Addition
func (v Vec2) Add(other Vec2) Vec2 {
	return Vec2{
		X: v.X + other.X,
		Y: v.Y + other.Y,
	}
}

// Subtraction
func (v Vec2) Sub(other Vec2) Vec2 {
	return Vec2{
		X: v.X - other.X,
		Y: v.Y - other.Y,
	}
}

// Componentwise multiplication
func (v Vec2) Mul(other Vec2) Vec2 {
	return Vec2{
		X: v.X * other.X,
		Y: v.Y * other.Y,
	}
}

// Componentwise division
func (v Vec2) Div(other Vec2) Vec2 {
	return Vec2{
		X: v.X / other.X,
		Y: v.Y / other.Y,
	}
}
