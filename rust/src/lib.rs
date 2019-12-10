use statrs::function::erf::erf;
use std::f64::consts::PI;

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
        ((value - self.mean).powi(2) / (2.0 * self.std_dev.powi(2))).exp()
            / (self.std_dev * (2.0 * PI).sqrt())
    }

    fn cdf(&self, value: f64) -> f64 {
        0.5 * (1.0 + erf((value - self.mean) / (self.std_dev * 2.0f64.sqrt())))
    }
}

pub fn make_std_norm_dist() -> NormalDistribution {
    NormalDistribution {
        mean: 1.0,
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
        option_type: &OptionType,
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

    pub fn price(&self, params: &BlackSholeModelParams, option_type: &OptionType) -> f64 {
        params.s * self.delta(params, option_type) - self.rho(params, option_type) / params.t
    }

    pub fn delta(&self, params: &BlackSholeModelParams, option_type: &OptionType) -> f64 {
        if let OptionType::Call = option_type {
            self.dist.cdf(params.d1())
        } else {
            -self.dist.cdf(-params.d1())
        }
    }

    pub fn gamma(&self, params: &BlackSholeModelParams, _option_type: &OptionType) -> f64 {
        self.dist.pdf(params.d1()) / (params.s * params.v * params.t.sqrt())
    }

    pub fn theta(&self, params: &BlackSholeModelParams, option_type: &OptionType) -> f64 {
        -params.v * 0.5 * self.vega(params, option_type) / params.t
            - params.r * self.rho(params, option_type) / params.t
    }

    pub fn vega(&self, params: &BlackSholeModelParams, _option_type: &OptionType) -> f64 {
        params.s * self.dist.pdf(params.d1()) * params.t.sqrt()
    }

    pub fn rho(&self, params: &BlackSholeModelParams, option_type: &OptionType) -> f64 {
        if let OptionType::Call = option_type {
            params.k * params.t * (-params.r * params.t).exp() * self.dist.cdf(params.d2())
        } else {
            -params.k * params.t * (-params.r * params.t).exp() * self.dist.cdf(-params.d2())
        }
    }
}
