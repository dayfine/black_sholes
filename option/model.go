package option

import "math"

type OptionType bool

const (
	CALL OptionType = true
	PUT  OptionType = false
)

// A probablity distribution of a value.
type Distribution interface {
	// Probability density function.
	Pdf(value float64) float64
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

func (d *NormDist) Pdf(value float64) float64 {
	return math.Exp(-math.Pow(value-d.mean, 2)/(2*d.stdDev*d.stdDev)) / (d.stdDev * math.Sqrt(math.Pi*2))
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

type BlackSholeModelResults struct {
	Price, Delta, Gamma, Theta, Vega, Rho float64
}

func (m *BlackSholeModel) Calc(p blackSholeModelParams, optionType OptionType) BlackSholeModelResults {
	return BlackSholeModelResults{
		Price: m.Price(p, optionType),
		Delta: m.Delta(p, optionType),
		Gamma: m.Gamma(p, optionType),
		Theta: m.Theta(p, optionType),
		Vega:  m.Vega(p, optionType),
		Rho:   m.Rho(p, optionType),
	}
}

func (m *BlackSholeModel) Price(p blackSholeModelParams, optionType OptionType) float64 {
	return p.S*m.Delta(p, optionType) - m.Rho(p, optionType)/p.T
}

func (m *BlackSholeModel) Delta(p blackSholeModelParams, optionType OptionType) float64 {
	if optionType == CALL {
		return m.dist.Cdf(p.d1())
	} else {
		return -m.dist.Cdf(-p.d1())
	}
}

func (m *BlackSholeModel) Gamma(p blackSholeModelParams, optionType OptionType) float64 {
	return m.dist.Pdf(p.d1()) / (p.S * p.V * math.Sqrt(p.T))
}

func (m *BlackSholeModel) Theta(p blackSholeModelParams, optionType OptionType) float64 {
	return -p.V*.5*m.Vega(p, optionType)/p.T - p.R*m.Rho(p, optionType)/p.T
}

func (m *BlackSholeModel) Vega(p blackSholeModelParams, optionType OptionType) float64 {
	return p.S * m.dist.Pdf(p.d1()) * math.Sqrt(p.T)
}

func (m *BlackSholeModel) Rho(p blackSholeModelParams, optionType OptionType) float64 {
	if optionType == CALL {
		return p.K * p.T * math.Exp(-p.R*p.T) * m.dist.Cdf(p.d2())
	} else {
		return -p.K * p.T * math.Exp(-p.R*p.T) * m.dist.Cdf(-p.d2())
	}
}
