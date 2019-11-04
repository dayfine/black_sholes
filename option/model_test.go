package option

import "testing"

func ExpectEq(t *testing.T, got, expected interface{}) {
	if expected != got {
		t.Helper()
		t.Errorf("Got: %T[%+v]\nExpected: %T[%+v]", got, got, expected, expected)
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
