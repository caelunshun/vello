// Applies sRGB transfer function to the RGB
// components of the vector. Alpha component is left
// untouched.
fn apply_srgb_transfer_function(color: vec4<f32>) -> vec4<f32> {
    let cutoff = color.rgb < vec3<f32>(0.0031308);
    let higher = 1.055 * pow(color.rgb, vec3<f32>(1.0 / 2.4)) - 0.055;
    let lower = color.rgb * 12.92;
    let result = mix(higher, lower, vec3<f32>(cutoff));
    return vec4<f32>(result, color.a);
}

// Applies inverse sRGB transfer function to the RGB
// components of the vector. Alpha component is left
// untouched.
fn inverse_srgb_transfer_function(color: vec4<f32>) -> vec4<f32> {
    let cutoff = color.rgb < vec3<f32>(0.04045);
    let higher = pow((color.rgb + vec3<f32>(0.055)) / vec3<f32>(1.055), vec3<f32>(2.4));
    let lower = color.rgb / vec3<f32>(12.92);

    let result = mix(higher, lower, vec3<f32>(cutoff));
    return vec4<f32>(result, color.a);
}
