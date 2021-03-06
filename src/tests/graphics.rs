use crate::tests;
use crate::*;

// use std::path;

#[test]
fn image_encode() {
    let (c, _e) = &mut tests::make_context();
    let image = graphics::Image::new(c, "/player.png").unwrap();
    image
        .encode(c, graphics::ImageFormat::Png, "/encode_test.png")
        .unwrap();
}

#[test]
fn save_screenshot() {
    let (c, _e) = &mut tests::make_context();
    graphics::clear(c, graphics::Color::new(0.1, 0.2, 0.3, 1.0));
    // Draw a triangle on it so you can see which way is right-side up.
    let tri_mesh = graphics::Mesh::new_polygon(
        c,
        graphics::DrawMode::stroke(10.0),
        &[
            graphics::Point2::new(100.0, 100.0),
            graphics::Point2::new(100.0, 200.0),
            graphics::Point2::new(200.0, 100.0),
        ],
        graphics::WHITE,
    )
    .unwrap();
    graphics::draw(c, &tri_mesh, graphics::DrawParam::default()).unwrap();
    graphics::present(c).unwrap();
    let screenshot = graphics::screenshot(c).unwrap();
    screenshot
        .encode(c, graphics::ImageFormat::Png, "/screenshot_test.png")
        .unwrap();
}

#[test]
fn load_images() {
    let (c, _e) = &mut tests::make_context();
    let image = graphics::Image::new(c, "/player.png").unwrap();
    image
        .encode(c, graphics::ImageFormat::Png, "/player_save_test.png")
        .unwrap();
    let _i2 = graphics::Image::new(c, "/player_save_test.png").unwrap();
}

#[test]
fn sanity_check_window_sizes() {
    let (c, e) = &mut tests::make_context();

    // Make sure that window sizes are what we ask for, and not what hidpi gives us.
    let w = c.conf.window_mode.width;
    let h = c.conf.window_mode.height;
    let size = graphics::drawable_size(c);
    assert_eq!(w, size.0);
    assert_eq!(h, size.1);

    let outer_size = graphics::size(c);
    assert!(size.0 <= outer_size.0);
    assert!(size.1 <= outer_size.1);

    // Make sure resizing the window works.
    let w = 100.0;
    let h = 200.0;
    graphics::set_drawable_size(c, w, h).unwrap();
    // ahahaha this apparently REQUIRES a delay between setting
    // the size and it actually altering, at least on Linux X11
    std::thread::sleep(std::time::Duration::from_millis(100));
    // Maybe we need to run the event pump too?  It seems VERY flaky.
    // Sometimes you need one, sometimes you need both...
    e.poll_events(|event| {
        c.process_event(&event);
    });

    let size = graphics::drawable_size(c);
    assert_eq!(w, size.0);
    assert_eq!(h, size.1);
}

/// Ensure that the transform stack applies operations in the correct order.
#[test]
fn test_transform_stack_order() {
    let (ctx, _e) = &mut tests::make_context();
    let p1 = graphics::DrawParam::default();
    let p2 = graphics::DrawParam::default();
    let t1 = p1.to_matrix();
    let t2 = p2.to_matrix();
    graphics::push_transform(ctx, Some(t1));
    graphics::mul_transform(ctx, t2);
    let res = crate::nalgebra::Matrix4::<f32>::from(graphics::transform(ctx));
    let m1: crate::nalgebra::Matrix4<f32> = t1.into();
    let m2: crate::nalgebra::Matrix4<f32> = t2.into();
    assert_eq!(res, m2 * m1);
}
