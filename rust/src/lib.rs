extern crate statrs;

use statrs::function::erf::erf;
use std::f64::consts::PI;

#[derive(Copy, Clone)]
pub enum OptionType {
    Call,
    Put,
}

pub struct NormalDistribution {
    mean: f64,
    std_dev: f64,
}

impl NormalDistribution {
    fn pdf(&self, value: f64) -> f64 {
        (-(value - self.mean).powi(2) / (2.0 * self.std_dev.powi(2))).exp()
            / (self.std_dev * (2.0 * PI).sqrt())
    }

    fn cdf(&self, value: f64) -> f64 {
        0.5 * (1.0 + erf((value - self.mean) / (self.std_dev * 2.0f64.sqrt())))
    }
}

pub fn make_std_norm_dist() -> NormalDistribution {
    NormalDistribution {
        mean: 0.0,
        std_dev: 1.0,
    }
}

pub struct BlackSholeModelParams {
    pub k: f64, // Exercie price
    pub s: f64, // Underlying price
    pub t: f64, // Years until expiration
    pub r: f64, // Applicable interest rate
    pub v: f64, // Future realized volatilty of the underlying
}

impl BlackSholeModelParams {
    fn d1(&self) -> f64 {
        ((self.s / self.k).ln() + (self.r + 0.5 * self.v.powi(2)) * self.t)
            / (self.v * self.t.sqrt())
    }

    fn d2(&self) -> f64 {
        ((self.s / self.k).ln() + (self.r - 0.5 * self.v.powi(2)) * self.t)
            / (self.v * self.t.sqrt())
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct BlackSholeModelResults {
    pub price: f64,
    pub delta: f64,
    pub gamma: f64,
    pub theta: f64,
    pub vega: f64,
    pub rho: f64,
}

pub struct BlackSholeModel {
    dist: NormalDistribution,
}

pub fn make_black_shole_model(dist: NormalDistribution) -> BlackSholeModel {
    BlackSholeModel { dist }
}

impl BlackSholeModel {
    pub fn calc(
        &self,
        params: &BlackSholeModelParams,
        option_type: OptionType,
    ) -> BlackSholeModelResults {
        BlackSholeModelResults {
            price: self.price(params, option_type),
            delta: self.delta(params, option_type),
            gamma: self.gamma(params, option_type),
            theta: self.theta(params, option_type),
            vega: self.vega(params, option_type),
            rho: self.rho(params, option_type),
        }
    }

    pub fn price(&self, params: &BlackSholeModelParams, option_type: OptionType) -> f64 {
        params.s * self.delta(params, option_type) - self.rho(params, option_type) / params.t
    }

    pub fn delta(&self, params: &BlackSholeModelParams, option_type: OptionType) -> f64 {
        if let OptionType::Call = option_type {
            self.dist.cdf(params.d1())
        } else {
            -self.dist.cdf(-params.d1())
        }
    }

    pub fn gamma(&self, params: &BlackSholeModelParams, _option_type: OptionType) -> f64 {
        self.dist.pdf(params.d1()) / (params.s * params.v * params.t.sqrt())
    }

    pub fn theta(&self, params: &BlackSholeModelParams, option_type: OptionType) -> f64 {
        -params.v * 0.5 * self.vega(params, option_type) / params.t
            - params.r * self.rho(params, option_type) / params.t
    }

    pub fn vega(&self, params: &BlackSholeModelParams, _option_type: OptionType) -> f64 {
        params.s * self.dist.pdf(params.d1()) * params.t.sqrt()
    }

    pub fn rho(&self, params: &BlackSholeModelParams, option_type: OptionType) -> f64 {
        if let OptionType::Call = option_type {
            params.k * params.t * (-params.r * params.t).exp() * self.dist.cdf(params.d2())
        } else {
            -params.k * params.t * (-params.r * params.t).exp() * self.dist.cdf(-params.d2())
        }
    }
}

#[cfg(test)]
mod norm_dist_test {
    use super::*;

    #[test]
    fn test_pdf() {
        let dist = make_std_norm_dist();

        assert_eq!(dist.pdf(0.0), 0.3989422804014327);
        assert_eq!(dist.pdf(0.1), 0.3969525474770118);
        assert_eq!(dist.pdf(0.5), 0.3520653267642995);
        assert_eq!(dist.pdf(1.0), 0.24197072451914337);
        assert_eq!(dist.pdf(-1.0), 0.24197072451914337);
        assert_eq!(dist.pdf(3.0), 0.0044318484119380075);
        assert_eq!(dist.pdf(-5.0), 1.4867195147342979e-06);
    }

    #[test]
    fn test_cdf() {
        let dist = make_std_norm_dist();

        assert_eq!(dist.cdf(0.0), 0.5);
        assert_eq!(dist.cdf(0.1), 0.539827837277029);
        assert_eq!(dist.cdf(0.5), 0.6914624612740131);
        assert_eq!(dist.cdf(1.0), 0.8413447460549428);
        assert_eq!(dist.cdf(-1.0), 0.15865525394505725);
        assert_eq!(dist.cdf(3.0), 0.9986501019684255);
        assert_eq!(dist.cdf(-5.0), 2.8665157186802404e-07);
    }
}


#[cfg(test)]
mod black_sholes_test {
    use super::*;

    fn get_params() -> BlackSholeModelParams{
        BlackSholeModelParams {
            k: 100.0,
            t: 1.0,
            r: 0.05,
            s: 100.0,
            v: 0.2,
        }
    }

    #[test]
    fn test_call_option() {
        let model = make_black_shole_model(make_std_norm_dist());

        assert_eq!(model.calc(&get_params(), OptionType::Call), BlackSholeModelResults{
            price: 10.450583572185565,
            delta: 0.6368306511756191,
            gamma: 0.018762017345846895,
            theta: -6.414027546438197,
            vega:  37.52403469169379,
            rho:   53.232481545376345,
        });
    }

    #[test]
    fn test_put_option() {
        let model = make_black_shole_model(make_std_norm_dist());

        assert_eq!(model.calc(&get_params(), OptionType::Put), BlackSholeModelResults{
            price: 5.573526022256971,
            delta: -0.3631693488243809,
            gamma: 0.018762017345846895,
            theta: -1.6578804239346265,
            vega:  37.52403469169379,
            rho:   -41.89046090469506,
        });
    }
}
