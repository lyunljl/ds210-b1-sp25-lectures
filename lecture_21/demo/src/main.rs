use plotters::prelude::*;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("plotters-doc-data/0.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;  // Set white background

    let root = root.titled("3D Plotting", ("Arial", 20).into_font())?;
    
    let mut chart = ChartBuilder::on(&root)
        .build_cartesian_3d(-10.0..10.0, -10.0..10.0, -10.0..10.0)?;
    
    chart.configure_axes().draw()?;
    
    // Draw a red circle parallel to XOZ panel
    chart.draw_series(LineSeries::new(
        (-314..314).map(|a| a as f64 / 100.0).map(|a| (8.0 * a.cos(), 0.0, 8.0 *a.sin())),
        &RED,
    ))?;
    // Draw a green circle parallel to YOZ panel
    chart.draw_series(LineSeries::new(
        (-314..314).map(|a| a as f64 / 100.0).map(|a| (0.0, 8.0 * a.cos(), 8.0 *a.sin())),
        &GREEN,
    ))?;
    
    Ok(())
}
