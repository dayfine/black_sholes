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
		params    blackSholeModelParams
		callPrice float64
		putPrice  float64
	}{
		{
			blackSholeModelParams{
				K: 250.0,
				T: 1.0,
				R: 0.03,
				S: 300.0,
				V: 0.15,
			},
			58.81976813699316,
			1.431151524120203,
		},
	}

	model := MakeBlackSholeModel(MakeStdNormDist())

	for _, test := range tests {
		ExpectEq(t, model.callPrice(test.params), test.callPrice)
		ExpectEq(t, model.putPrice(test.params), test.putPrice)
	}
}
