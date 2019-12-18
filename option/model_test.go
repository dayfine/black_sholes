package option

import (
	"testing"

	. "github.com/dayfine/base-go/base/testing"
)

func TestNormDist(t *testing.T) {
	tests := []struct {
		input, pdf, cdf float64
	}{
		{
			0.0, 0.3989422804014327, 0.5,
		},
		{
			0.1, 0.3969525474770118, 0.539827837277029,
		},
		{
			0.5, 0.3520653267642995, 0.6914624612740131,
		},
		{
			1.0, 0.24197072451914337, 0.8413447460685429,
		},
		{
			-1.0, 0.24197072451914337, 0.15865525393145707,
		},
		{
			3.0, 0.0044318484119380075, 0.9986501019683699,
		},
		{
			-5.0, 1.4867195147342979e-06, 2.8665157186802404e-07,
		},
	}

	dist := MakeStdNormDist()

	for _, test := range tests {
		ExpectEq(t, dist.Pdf(test.input), test.pdf)
		ExpectEq(t, dist.Cdf(test.input), test.cdf)
	}
}

func TestBlackSholeModel(t *testing.T) {
	tests := []struct {
		params blackSholeModelParams
		call   BlackSholeModelResults
		put    BlackSholeModelResults
	}{
		{
			blackSholeModelParams{
				K: 100.0,
				T: 1.0,
				R: 0.05,
				S: 100.0,
				V: 0.2,
			},
			BlackSholeModelResults{
				Price: 10.450583572185565,
				Delta: 0.6368306511756191,
				Gamma: 0.018762017345846895,
				Theta: -6.414027546438197,
				Vega:  37.52403469169379,
				Rho:   53.232481545376345,
			},
			BlackSholeModelResults{
				Price: 5.573526022256971,
				Delta: -0.3631693488243809,
				Gamma: 0.018762017345846895,
				Theta: -1.6578804239346265,
				Vega:  37.52403469169379,
				Rho:   -41.89046090469506,
			},
		},
	}

	model := MakeBlackSholeModel(MakeStdNormDist())

	for _, test := range tests {
		ExpectEq(t, model.Calc(test.params, CALL), test.call)
		ExpectEq(t, model.Calc(test.params, PUT), test.put)
	}
}
