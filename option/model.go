package option

import "math"

// A probablity distribution of a value.
type Distribution interface {
	// Cumulative distribution function.
	Cdf(value float64) float64
}

func MakeStdNormDist() *NormDist {
	return &NormDist{
		mean:   0.0,
		stdDev: 1.0,
	}
}

type NormDist struct {
	mean, stdDev float64
}

func (d *NormDist) Cdf(value float64) float64 {
	return 0.5 * (1 + math.Erf((value-d.mean)/(d.stdDev*math.Sqrt2)))
}

type blackSholeModelParams struct {
	K, // ExerciePrice
	S, // UnderlyingPrice
	T, // YearsToExpiration
	R, // ApplicableInterestRate
	V float64 // Future realized volatilty of the underlying
}

func (p *blackSholeModelParams) d1() float64 {
	return (math.Log(p.S/p.K) + (p.R+0.5*p.V*p.V)*p.T) / (p.V * (math.Sqrt(p.T)))
}

func (p *blackSholeModelParams) d2() float64 {
	return (math.Log(p.S/p.K) + (p.R-0.5*p.V*p.V)*p.T) / (p.V * (math.Sqrt(p.T)))
}

func MakeBlackSholeModel(dist Distribution) *BlackSholeModel {
	return &BlackSholeModel{
		dist: dist,
	}
}

type BlackSholeModel struct {
	dist Distribution
}

func (m *BlackSholeModel) callPrice(p blackSholeModelParams) float64 {
	return p.S*m.dist.Cdf(p.d1()) - p.K*math.Exp(-p.R*p.T)*m.dist.Cdf(p.d2())
}

func (m *BlackSholeModel) putPrice(p blackSholeModelParams) float64 {
	return -p.S*m.dist.Cdf(-p.d1()) + p.K*math.Exp(-p.R*p.T)*m.dist.Cdf(-p.d2())
}
