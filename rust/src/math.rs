#[allow(clippy::cast_precision_loss)]
pub fn average_f32(xs: &[f32]) -> Option<f32> {
    if xs.is_empty() {
        None
    } else {
        Some((xs.iter().sum::<f32>()) / (xs.len() as f32))
    }
}

#[allow(clippy::cast_precision_loss)]
pub fn std_dev_f32(xs: &[f32]) -> Option<f32> {
    average_f32(xs).map(|average| {
        let variance: f32 = xs
            .iter()
            .map(|value| {
                let delta = average - value;
                delta * delta
            })
            .sum::<f32>()
            / (xs.len() as f32);
        variance.sqrt()
    })
}

/* https://en.wikipedia.org/wiki/Feature_scaling */
pub fn unit_scale(xs: &mut Vec<f32>) -> Option<()> {
    if let (Some(xs_avg), Some(xs_std)) = (average_f32(&xs), std_dev_f32(&xs))
    {
        if xs_std != 0.0 {
            for x in xs.iter_mut() {
                *x = (*x - xs_avg) / xs_std;
            }
            return Some(());
        }
    }
    None
}
