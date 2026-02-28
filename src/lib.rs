use centaur_technical_indicators::basic_indicators::bulk as basic_indicators;
use centaur_technical_indicators::candle_indicators::bulk as candle_indicators;
use centaur_technical_indicators::chart_trends;
use centaur_technical_indicators::correlation_indicators::bulk as correlation_indicators;
use centaur_technical_indicators::momentum_indicators::bulk as momentum_indicators;
use centaur_technical_indicators::moving_average::bulk as moving_average;
use centaur_technical_indicators::other_indicators::bulk as other_indicators;
use centaur_technical_indicators::strength_indicators::bulk as strength_indicators;
use centaur_technical_indicators::trend_indicators::bulk as trend_indicators;
use centaur_technical_indicators::volatility_indicators::bulk as volatility_indicators;
use centaur_technical_indicators::{AbsDevConfig, CentralPoint, ConstantModelType, DeviationAggregate, DeviationModel, MovingAverageType, Position};
use centaur_technical_indicators::chart_trends::TrendBreakConfig;

mod data_constants;

// Constants for data sizes
const SMALL_SIZE: usize = 100;
const MEDIUM_SIZE: usize = 1000;

// Helper functions for accessing different data sizes
pub fn get_small_data() -> &'static [f64] {
    &data_constants::PRICES[..SMALL_SIZE]
}

pub fn get_medium_data() -> &'static [f64] {
    &data_constants::PRICES[..MEDIUM_SIZE]
}

pub fn get_large_data() -> &'static [f64] {
    &data_constants::PRICES
}

pub fn get_small_high() -> &'static [f64] {
    &data_constants::HIGH[..SMALL_SIZE]
}

pub fn get_medium_high() -> &'static [f64] {
    &data_constants::HIGH[..MEDIUM_SIZE]
}

pub fn get_large_high() -> &'static [f64] {
    &data_constants::HIGH
}

pub fn get_small_low() -> &'static [f64] {
    &data_constants::LOW[..SMALL_SIZE]
}

pub fn get_medium_low() -> &'static [f64] {
    &data_constants::LOW[..MEDIUM_SIZE]
}

pub fn get_large_low() -> &'static [f64] {
    &data_constants::LOW
}

pub fn get_small_close() -> &'static [f64] {
    &data_constants::CLOSE[..SMALL_SIZE]
}

pub fn get_medium_close() -> &'static [f64] {
    &data_constants::CLOSE[..MEDIUM_SIZE]
}

pub fn get_large_close() -> &'static [f64] {
    &data_constants::CLOSE
}

pub fn get_small_open() -> &'static [f64] {
    &data_constants::OPEN[..SMALL_SIZE]
}

pub fn get_medium_open() -> &'static [f64] {
    &data_constants::OPEN[..MEDIUM_SIZE]
}

pub fn get_large_open() -> &'static [f64] {
    &data_constants::OPEN
}

pub fn get_small_volume() -> &'static [f64] {
    &data_constants::VOLUME[..SMALL_SIZE]
}

pub fn get_medium_volume() -> &'static [f64] {
    &data_constants::VOLUME[..MEDIUM_SIZE]
}

pub fn get_large_volume() -> &'static [f64] {
    &data_constants::VOLUME
}

// Momentum indicators

pub fn compute_rsi() -> Vec<f64> {
    momentum_indicators::relative_strength_index(
        &data_constants::PRICES,
        ConstantModelType::SmoothedMovingAverage,
        14,
    ).unwrap()
}

pub fn compute_rsi_small() -> Vec<f64> {
    momentum_indicators::relative_strength_index(
        get_small_data(),
        ConstantModelType::SmoothedMovingAverage,
        14,
    ).unwrap()
}

pub fn compute_rsi_medium() -> Vec<f64> {
    momentum_indicators::relative_strength_index(
        get_medium_data(),
        ConstantModelType::SmoothedMovingAverage,
        14,
    ).unwrap()
}

pub fn compute_rsi_simple() -> Vec<f64> {
    momentum_indicators::relative_strength_index(
        &data_constants::PRICES,
        ConstantModelType::SimpleMovingAverage,
        14,
    ).unwrap()
}

pub fn compute_rsi_exp() -> Vec<f64> {
    momentum_indicators::relative_strength_index(
        &data_constants::PRICES,
        ConstantModelType::ExponentialMovingAverage,
        14,
    ).unwrap()
}

pub fn compute_rsi_7() -> Vec<f64> {
    momentum_indicators::relative_strength_index(
        &data_constants::PRICES,
        ConstantModelType::SmoothedMovingAverage,
        7,
    ).unwrap()
}

pub fn compute_rsi_21() -> Vec<f64> {
    momentum_indicators::relative_strength_index(
        &data_constants::PRICES,
        ConstantModelType::SmoothedMovingAverage,
        21,
    ).unwrap()
}

pub fn compute_so() -> Vec<f64> {
    momentum_indicators::stochastic_oscillator(&data_constants::PRICES, 14).unwrap()
}

pub fn compute_so_small() -> Vec<f64> {
    momentum_indicators::stochastic_oscillator(get_small_data(), 14).unwrap()
}

pub fn compute_so_medium() -> Vec<f64> {
    momentum_indicators::stochastic_oscillator(get_medium_data(), 14).unwrap()
}

pub fn compute_so_5() -> Vec<f64> {
    momentum_indicators::stochastic_oscillator(&data_constants::PRICES, 5).unwrap()
}

pub fn compute_slow_so() -> Vec<f64> {
    momentum_indicators::slow_stochastic(
        &data_constants::PRICES,
        ConstantModelType::SimpleMovingAverage,
        14,
    ).unwrap()
}

pub fn compute_slow_so_small() -> Vec<f64> {
    momentum_indicators::slow_stochastic(
        get_small_data(),
        ConstantModelType::SimpleMovingAverage,
        14,
    ).unwrap()
}

pub fn compute_slow_so_medium() -> Vec<f64> {
    momentum_indicators::slow_stochastic(
        get_medium_data(),
        ConstantModelType::SimpleMovingAverage,
        14,
    ).unwrap()
}

pub fn compute_slow_so_exp() -> Vec<f64> {
    momentum_indicators::slow_stochastic(
        &data_constants::PRICES,
        ConstantModelType::ExponentialMovingAverage,
        14,
    ).unwrap()
}

pub fn compute_slowest_so() -> Vec<f64> {
    momentum_indicators::slowest_stochastic(
        &data_constants::PRICES,
        ConstantModelType::SimpleMovingAverage,
        14,
    ).unwrap()
}

pub fn compute_slowest_so_small() -> Vec<f64> {
    momentum_indicators::slowest_stochastic(
        get_small_data(),
        ConstantModelType::SimpleMovingAverage,
        14,
    ).unwrap()
}

pub fn compute_slowest_so_medium() -> Vec<f64> {
    momentum_indicators::slowest_stochastic(
        get_medium_data(),
        ConstantModelType::SimpleMovingAverage,
        14,
    ).unwrap()
}

pub fn compute_williams_r() -> Vec<f64> {
    momentum_indicators::williams_percent_r(
        &data_constants::HIGH,
        &data_constants::LOW,
        &data_constants::CLOSE,
        14,
    ).unwrap()
}

pub fn compute_williams_r_small() -> Vec<f64> {
    momentum_indicators::williams_percent_r(
        get_small_high(),
        get_small_low(),
        get_small_close(),
        14,
    ).unwrap()
}

pub fn compute_williams_r_medium() -> Vec<f64> {
    momentum_indicators::williams_percent_r(
        get_medium_high(),
        get_medium_low(),
        get_medium_close(),
        14,
    ).unwrap()
}

pub fn compute_williams_r_7() -> Vec<f64> {
    momentum_indicators::williams_percent_r(
        &data_constants::HIGH,
        &data_constants::LOW,
        &data_constants::CLOSE,
        7,
    ).unwrap()
}

pub fn compute_williams_r_21() -> Vec<f64> {
    momentum_indicators::williams_percent_r(
        &data_constants::HIGH,
        &data_constants::LOW,
        &data_constants::CLOSE,
        21,
    ).unwrap()
}

pub fn compute_mfi() -> Vec<f64> {
    momentum_indicators::money_flow_index(&data_constants::PRICES, &data_constants::VOLUME, 14).unwrap()
}

pub fn compute_mfi_small() -> Vec<f64> {
    momentum_indicators::money_flow_index(get_small_data(), get_small_volume(), 14).unwrap()
}

pub fn compute_mfi_medium() -> Vec<f64> {
    momentum_indicators::money_flow_index(get_medium_data(), get_medium_volume(), 14).unwrap()
}

pub fn compute_mfi_7() -> Vec<f64> {
    momentum_indicators::money_flow_index(&data_constants::PRICES, &data_constants::VOLUME, 7).unwrap()
}

pub fn compute_roc() -> Vec<f64> {
    momentum_indicators::rate_of_change(&data_constants::PRICES).unwrap()
}

pub fn compute_roc_small() -> Vec<f64> {
    momentum_indicators::rate_of_change(get_small_data()).unwrap()
}

pub fn compute_roc_medium() -> Vec<f64> {
    momentum_indicators::rate_of_change(get_medium_data()).unwrap()
}

pub fn compute_obv() -> Vec<f64> {
    momentum_indicators::on_balance_volume(&data_constants::PRICES, &data_constants::VOLUME, 0.0).unwrap()
}

pub fn compute_obv_small() -> Vec<f64> {
    momentum_indicators::on_balance_volume(get_small_data(), get_small_volume(), 0.0).unwrap()
}

pub fn compute_obv_medium() -> Vec<f64> {
    momentum_indicators::on_balance_volume(get_medium_data(), get_medium_volume(), 0.0).unwrap()
}

pub fn compute_cci() -> Vec<f64> {
    momentum_indicators::commodity_channel_index(
        &data_constants::PRICES,
        ConstantModelType::SimpleMovingAverage,
        DeviationModel::StandardDeviation,
        0.015,
        14,
    ).unwrap()
}

pub fn compute_cci_small() -> Vec<f64> {
    momentum_indicators::commodity_channel_index(
        get_small_data(),
        ConstantModelType::SimpleMovingAverage,
        DeviationModel::StandardDeviation,
        0.015,
        14,
    ).unwrap()
}

pub fn compute_cci_medium() -> Vec<f64> {
    momentum_indicators::commodity_channel_index(
        get_medium_data(),
        ConstantModelType::SimpleMovingAverage,
        DeviationModel::StandardDeviation,
        0.015,
        14,
    ).unwrap()
}

pub fn compute_cci_exp() -> Vec<f64> {
    momentum_indicators::commodity_channel_index(
        &data_constants::PRICES,
        ConstantModelType::ExponentialMovingAverage,
        DeviationModel::StandardDeviation,
        0.015,
        14,
    ).unwrap()
}

pub fn compute_cci_mad() -> Vec<f64> {
    momentum_indicators::commodity_channel_index(
        &data_constants::PRICES,
        ConstantModelType::SimpleMovingAverage,
        DeviationModel::MeanAbsoluteDeviation,
        0.015,
        14,
    ).unwrap()
}

pub fn compute_cci_20() -> Vec<f64> {
    momentum_indicators::commodity_channel_index(
        &data_constants::PRICES,
        ConstantModelType::SimpleMovingAverage,
        DeviationModel::StandardDeviation,
        0.015,
        20,
    ).unwrap()
}

pub fn compute_mg_cci() -> Vec<(f64, f64)> {
    momentum_indicators::mcginley_dynamic_commodity_channel_index(
        &data_constants::PRICES,
        0.0,
        DeviationModel::MeanAbsoluteDeviation,
        0.015,
        14,
    ).unwrap()
}

pub fn compute_macd() -> Vec<f64> {
    momentum_indicators::macd_line(
        &data_constants::PRICES,
        7,
        ConstantModelType::SimpleMovingAverage,
        14,
        ConstantModelType::SimpleMovingAverage,
    ).unwrap()
}

pub fn compute_macd_small() -> Vec<f64> {
    momentum_indicators::macd_line(
        get_small_data(),
        7,
        ConstantModelType::SimpleMovingAverage,
        14,
        ConstantModelType::SimpleMovingAverage,
    ).unwrap()
}

pub fn compute_macd_medium() -> Vec<f64> {
    momentum_indicators::macd_line(
        get_medium_data(),
        7,
        ConstantModelType::SimpleMovingAverage,
        14,
        ConstantModelType::SimpleMovingAverage,
    ).unwrap()
}

pub fn compute_macd_exp() -> Vec<f64> {
    momentum_indicators::macd_line(
        &data_constants::PRICES,
        7,
        ConstantModelType::ExponentialMovingAverage,
        14,
        ConstantModelType::ExponentialMovingAverage,
    ).unwrap()
}

pub fn compute_macd_standard() -> Vec<f64> {
    momentum_indicators::macd_line(
        &data_constants::PRICES,
        12,
        ConstantModelType::ExponentialMovingAverage,
        26,
        ConstantModelType::ExponentialMovingAverage,
    ).unwrap()
}

pub fn compute_mg_macd() -> Vec<(f64, f64, f64)> {
    momentum_indicators::mcginley_dynamic_macd_line(&data_constants::PRICES, 7, 0.0, 14, 0.0).unwrap()
}

pub fn compute_co() -> Vec<(f64, f64)> {
    momentum_indicators::chaikin_oscillator(
        &data_constants::HIGH,
        &data_constants::LOW,
        &data_constants::CLOSE,
        &data_constants::VOLUME,
        7,
        14,
        0.0,
        ConstantModelType::SimpleMovingAverage,
        ConstantModelType::SimpleMovingAverage,
    ).unwrap()
}

pub fn compute_ppo() -> Vec<f64> {
    momentum_indicators::percentage_price_oscillator(
        &data_constants::PRICES,
        7,
        14,
        ConstantModelType::SimpleMovingAverage,
    ).unwrap()
}

pub fn compute_cmo() -> Vec<f64> {
    momentum_indicators::chande_momentum_oscillator(&data_constants::PRICES, 14).unwrap()
}

// Candle Indicators

pub fn compute_cnst_env() -> Vec<(f64, f64, f64)> {
    candle_indicators::moving_constant_envelopes(
        &data_constants::PRICES,
        ConstantModelType::SimpleMovingAverage,
        3.0,
        10,
    ).unwrap()
}

pub fn compute_mg_env() -> Vec<(f64, f64, f64)> {
    candle_indicators::mcginley_dynamic_envelopes(&data_constants::PRICES, 3.0, 0.0, 10).unwrap()
}

pub fn compute_cnst_bands() -> Vec<(f64, f64, f64)> {
    candle_indicators::moving_constant_bands(
        &data_constants::PRICES,
        ConstantModelType::SimpleMovingAverage,
        DeviationModel::StandardDeviation,
        2.0,
        5,
    ).unwrap()
}

pub fn compute_mg_bands() -> Vec<(f64, f64, f64)> {
    candle_indicators::mcginley_dynamic_bands(
        &data_constants::PRICES,
        DeviationModel::StandardDeviation,
        2.0,
        0.0,
        5,
    ).unwrap()
}

pub fn compute_icloud() -> Vec<(f64, f64, f64, f64, f64)> {
    candle_indicators::ichimoku_cloud(
        &data_constants::HIGH,
        &data_constants::LOW,
        &data_constants::CLOSE,
        5,
        10,
        15,
    ).unwrap()
}

pub fn compute_donchian_chnl() -> Vec<(f64, f64, f64)> {
    candle_indicators::donchian_channels(&data_constants::HIGH, &data_constants::LOW, 5).unwrap()
}

pub fn compute_keltner_chnl() -> Vec<(f64, f64, f64)> {
    candle_indicators::keltner_channel(
        &data_constants::HIGH,
        &data_constants::LOW,
        &data_constants::CLOSE,
        ConstantModelType::ExponentialMovingAverage,
        ConstantModelType::SimpleMovingAverage,
        2.0,
        5,
    ).unwrap()
}

pub fn compute_supertrend() -> Vec<f64> {
    candle_indicators::supertrend(
        &data_constants::HIGH,
        &data_constants::LOW,
        &data_constants::CLOSE,
        ConstantModelType::SimpleMovingAverage,
        2.0,
        5,
    ).unwrap()
}

// Trend Indicators

pub fn compute_aroon_up() -> Vec<f64> {
    trend_indicators::aroon_up(&data_constants::HIGH, 5).unwrap()
}

pub fn compute_aroon_down() -> Vec<f64> {
    trend_indicators::aroon_down(&data_constants::LOW, 5).unwrap()
}

pub fn compute_aroon_indicator() -> Vec<(f64, f64, f64)> {
    trend_indicators::aroon_indicator(&data_constants::HIGH, &data_constants::LOW, 5).unwrap()
}

pub fn compute_parabolic_tps() -> Vec<f64> {
    trend_indicators::parabolic_time_price_system(
        &data_constants::HIGH,
        &data_constants::LOW,
        0.02,
        0.2,
        0.02,
        Position::Long,
        0.0,
    ).unwrap()
}

pub fn compute_dir_mov_sys() -> Vec<(f64, f64, f64, f64)> {
    trend_indicators::directional_movement_system(
        &data_constants::HIGH,
        &data_constants::LOW,
        &data_constants::CLOSE,
        5,
        ConstantModelType::SimpleMovingAverage,
    ).unwrap()
}

pub fn compute_vpt() -> Vec<f64> {
    let reduced_volume: Vec<f64> = (0..2551).map(|i| data_constants::VOLUME[i]).collect();
    trend_indicators::volume_price_trend(&data_constants::PRICES, &reduced_volume, 0.0).unwrap()
}

pub fn compute_tsi() -> Vec<f64> {
    trend_indicators::true_strength_index(
        &data_constants::PRICES,
        ConstantModelType::SimpleMovingAverage,
        10,
        ConstantModelType::SimpleMovingAverage,
        5,
    ).unwrap()
}

// Strength Indicators

pub fn compute_ad() -> Vec<f64> {
    strength_indicators::accumulation_distribution(
        &data_constants::HIGH,
        &data_constants::LOW,
        &data_constants::CLOSE,
        &data_constants::VOLUME,
        0.0,
    ).unwrap()
}

pub fn compute_pvi() -> Vec<f64> {
    strength_indicators::positive_volume_index(&data_constants::CLOSE, &data_constants::VOLUME, 0.0).unwrap()
}

pub fn compute_nvi() -> Vec<f64> {
    strength_indicators::negative_volume_index(&data_constants::CLOSE, &data_constants::VOLUME, 0.0).unwrap()
}

pub fn compute_rvi() -> Vec<f64> {
    strength_indicators::relative_vigor_index(
        &data_constants::OPEN,
        &data_constants::HIGH,
        &data_constants::LOW,
        &data_constants::CLOSE,
        ConstantModelType::SimpleMovingAverage,
        10,
    ).unwrap()
}

// Other Indicators

pub fn compute_roi() -> Vec<(f64, f64)> {
    other_indicators::return_on_investment(&data_constants::PRICES, 1000.0).unwrap()
}

pub fn compute_tr() -> Vec<f64> {
    other_indicators::true_range(
        &data_constants::CLOSE,
        &data_constants::HIGH,
        &data_constants::LOW,
    ).unwrap()
}

pub fn compute_atr() -> Vec<f64> {
    other_indicators::average_true_range(
        &data_constants::CLOSE,
        &data_constants::HIGH,
        &data_constants::LOW,
        ConstantModelType::SimpleMovingAverage,
        5,
    ).unwrap()
}

pub fn compute_atr_small() -> Vec<f64> {
    other_indicators::average_true_range(
        get_small_close(),
        get_small_high(),
        get_small_low(),
        ConstantModelType::SimpleMovingAverage,
        5,
    ).unwrap()
}

pub fn compute_atr_medium() -> Vec<f64> {
    other_indicators::average_true_range(
        get_medium_close(),
        get_medium_high(),
        get_medium_low(),
        ConstantModelType::SimpleMovingAverage,
        5,
    ).unwrap()
}

pub fn compute_atr_14() -> Vec<f64> {
    other_indicators::average_true_range(
        &data_constants::CLOSE,
        &data_constants::HIGH,
        &data_constants::LOW,
        ConstantModelType::SimpleMovingAverage,
        14,
    ).unwrap()
}

pub fn compute_atr_exp() -> Vec<f64> {
    other_indicators::average_true_range(
        &data_constants::CLOSE,
        &data_constants::HIGH,
        &data_constants::LOW,
        ConstantModelType::ExponentialMovingAverage,
        14,
    ).unwrap()
}

pub fn compute_ibs() -> Vec<f64> {
    other_indicators::internal_bar_strength(
        &data_constants::HIGH,
        &data_constants::LOW,
        &data_constants::CLOSE,
    ).unwrap()
}

pub fn compute_pi() -> Vec<(f64, f64)> {
    other_indicators::positivity_indicator(
        &data_constants::OPEN,
        &data_constants::CLOSE,
        5,
        ConstantModelType::SimpleMovingAverage,
    ).unwrap()
}

// Basic Indicators

pub fn compute_mean() -> Vec<f64> {
    basic_indicators::mean(&data_constants::PRICES, 5).unwrap()
}

pub fn compute_mean_small() -> Vec<f64> {
    basic_indicators::mean(get_small_data(), 5).unwrap()
}

pub fn compute_mean_medium() -> Vec<f64> {
    basic_indicators::mean(get_medium_data(), 5).unwrap()
}

pub fn compute_mean_20() -> Vec<f64> {
    basic_indicators::mean(&data_constants::PRICES, 20).unwrap()
}

pub fn compute_median() -> Vec<f64> {
    basic_indicators::median(&data_constants::PRICES, 5).unwrap()
}

pub fn compute_median_small() -> Vec<f64> {
    basic_indicators::median(get_small_data(), 5).unwrap()
}

pub fn compute_median_medium() -> Vec<f64> {
    basic_indicators::median(get_medium_data(), 5).unwrap()
}

pub fn compute_median_20() -> Vec<f64> {
    basic_indicators::median(&data_constants::PRICES, 20).unwrap()
}

pub fn compute_mode() -> Vec<f64> {
    basic_indicators::mode(&data_constants::PRICES, 5).unwrap()
}

pub fn compute_log() -> Vec<f64> {
    basic_indicators::log(&data_constants::PRICES).unwrap()
}

pub fn compute_log_small() -> Vec<f64> {
    basic_indicators::log(get_small_data()).unwrap()
}

pub fn compute_log_medium() -> Vec<f64> {
    basic_indicators::log(get_medium_data()).unwrap()
}

pub fn compute_log_diff() -> Vec<f64> {
    basic_indicators::log_difference(&data_constants::PRICES).unwrap()
}

pub fn compute_var() -> Vec<f64> {
    basic_indicators::variance(&data_constants::PRICES, 5).unwrap()
}

pub fn compute_var_small() -> Vec<f64> {
    basic_indicators::variance(get_small_data(), 5).unwrap()
}

pub fn compute_var_medium() -> Vec<f64> {
    basic_indicators::variance(get_medium_data(), 5).unwrap()
}

pub fn compute_var_20() -> Vec<f64> {
    basic_indicators::variance(&data_constants::PRICES, 20).unwrap()
}

pub fn compute_stdev() -> Vec<f64> {
    basic_indicators::standard_deviation(&data_constants::PRICES, 5).unwrap()
}

pub fn compute_stdev_small() -> Vec<f64> {
    basic_indicators::standard_deviation(get_small_data(), 5).unwrap()
}

pub fn compute_stdev_medium() -> Vec<f64> {
    basic_indicators::standard_deviation(get_medium_data(), 5).unwrap()
}

pub fn compute_stdev_20() -> Vec<f64> {
    basic_indicators::standard_deviation(&data_constants::PRICES, 20).unwrap()
}

pub fn compute_mean_abs_dev() -> Vec<f64> {
    basic_indicators::absolute_deviation(&data_constants::PRICES, 5, AbsDevConfig {
        center: CentralPoint::Mean,
        aggregate: DeviationAggregate::Mean,
    }).unwrap()
}

pub fn compute_median_abs_dev() -> Vec<f64> {
    basic_indicators::absolute_deviation(&data_constants::PRICES, 5, AbsDevConfig {
        center: CentralPoint::Median,
        aggregate: DeviationAggregate::Median,
    }).unwrap()
}

pub fn compute_mode_abs_dev() -> Vec<f64> {
    basic_indicators::absolute_deviation(&data_constants::PRICES, 5, AbsDevConfig {
        center: CentralPoint::Mode,
        aggregate: DeviationAggregate::Mode,
    }).unwrap()
}

// Chart trends

pub fn compute_peaks() -> Vec<(f64, usize)> {
    chart_trends::peaks(&data_constants::PRICES, 5, 2).unwrap()
}

pub fn compute_valleys() -> Vec<(f64, usize)> {
    chart_trends::valleys(&data_constants::PRICES, 5, 2).unwrap()
}

pub fn compute_peak_trend() -> (f64, f64) {
    chart_trends::peak_trend(&data_constants::PRICES, 5).unwrap()
}

pub fn compute_valley_trend() -> (f64, f64) {
    chart_trends::valley_trend(&data_constants::PRICES, 5).unwrap()
}

pub fn compute_overall_trend() -> (f64, f64) {
    chart_trends::overall_trend(&data_constants::PRICES).unwrap()
}

pub fn compute_break_down_trends() -> Vec<(usize, usize, f64, f64)> {
    chart_trends::break_down_trends(
        &data_constants::PRICES,
        TrendBreakConfig {
            max_outliers: 1,
            soft_adj_r_squared_minimum: 0.75,
            hard_adj_r_squared_minimum: 1.0,
            soft_rmse_multiplier: 0.5,
            hard_rmse_multiplier: 1.5,
            soft_durbin_watson_min: 2.0,
            soft_durbin_watson_max: 3.0,
            hard_durbin_watson_min: 2.0,
            hard_durbin_watson_max: 3.0,
        },
    ).unwrap()
}

// Correlation Indicators

pub fn compute_corr() -> Vec<f64> {
    correlation_indicators::correlate_asset_prices(
        &data_constants::HIGH,
        &data_constants::LOW,
        ConstantModelType::SimpleMovingAverage,
        DeviationModel::StandardDeviation,
        5,
    ).unwrap()
}

// Moving Average

pub fn compute_ma() -> Vec<f64> {
    moving_average::moving_average(&data_constants::PRICES, MovingAverageType::Simple, 5).unwrap()
}

pub fn compute_ma_small() -> Vec<f64> {
    moving_average::moving_average(get_small_data(), MovingAverageType::Simple, 5).unwrap()
}

pub fn compute_ma_medium() -> Vec<f64> {
    moving_average::moving_average(get_medium_data(), MovingAverageType::Simple, 5).unwrap()
}

pub fn compute_ma_10() -> Vec<f64> {
    moving_average::moving_average(&data_constants::PRICES, MovingAverageType::Simple, 10).unwrap()
}

pub fn compute_ma_20() -> Vec<f64> {
    moving_average::moving_average(&data_constants::PRICES, MovingAverageType::Simple, 20).unwrap()
}

pub fn compute_ma_50() -> Vec<f64> {
    moving_average::moving_average(&data_constants::PRICES, MovingAverageType::Simple, 50).unwrap()
}

pub fn compute_sma() -> Vec<f64> {
    moving_average::moving_average(&data_constants::PRICES, MovingAverageType::Smoothed, 5).unwrap()
}

pub fn compute_sma_small() -> Vec<f64> {
    moving_average::moving_average(get_small_data(), MovingAverageType::Smoothed, 5).unwrap()
}

pub fn compute_sma_medium() -> Vec<f64> {
    moving_average::moving_average(get_medium_data(), MovingAverageType::Smoothed, 5).unwrap()
}

pub fn compute_sma_20() -> Vec<f64> {
    moving_average::moving_average(&data_constants::PRICES, MovingAverageType::Smoothed, 20).unwrap()
}

pub fn compute_ema() -> Vec<f64> {
    moving_average::moving_average(&data_constants::PRICES, MovingAverageType::Exponential, 5).unwrap()
}

pub fn compute_ema_small() -> Vec<f64> {
    moving_average::moving_average(get_small_data(), MovingAverageType::Exponential, 5).unwrap()
}

pub fn compute_ema_medium() -> Vec<f64> {
    moving_average::moving_average(get_medium_data(), MovingAverageType::Exponential, 5).unwrap()
}

pub fn compute_ema_12() -> Vec<f64> {
    moving_average::moving_average(&data_constants::PRICES, MovingAverageType::Exponential, 12).unwrap()
}

pub fn compute_ema_26() -> Vec<f64> {
    moving_average::moving_average(&data_constants::PRICES, MovingAverageType::Exponential, 26).unwrap()
}

pub fn compute_mg_dyn() -> Vec<f64> {
    moving_average::mcginley_dynamic(&data_constants::PRICES, 0.0, 5).unwrap()
}

pub fn compute_mg_dyn_small() -> Vec<f64> {
    moving_average::mcginley_dynamic(get_small_data(), 0.0, 5).unwrap()
}

pub fn compute_mg_dyn_medium() -> Vec<f64> {
    moving_average::mcginley_dynamic(get_medium_data(), 0.0, 5).unwrap()
}

pub fn compute_mg_dyn_20() -> Vec<f64> {
    moving_average::mcginley_dynamic(&data_constants::PRICES, 0.0, 20).unwrap()
}

// Volatility Indicators

pub fn compute_ulcer() -> Vec<f64> {
    volatility_indicators::ulcer_index(&data_constants::PRICES, 5).unwrap()
}

pub fn compute_vs() -> Vec<f64> {
    volatility_indicators::volatility_system(
        &data_constants::HIGH,
        &data_constants::LOW,
        &data_constants::CLOSE,
        5,
        3.0,
        centaur_technical_indicators::ConstantModelType::SimpleMovingAverage,
    ).unwrap()
}
