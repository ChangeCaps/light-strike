#version 450

out vec4 color;

buffer object_length_buffer {
    int object_length[];
};

buffer object_position_buffer {
    vec2 object_position[];
};

buffer light_position_buffer {
    vec2 light_position[];
};

buffer light_buffer {
    vec4 lights[];
};

uniform vec2 resolution;
uniform vec2 camera_position;
uniform float camera_size;


bool ray_line(vec2 ray_origin, vec2 ray_direction, vec2 line_0, vec2 line_1, float max_dist, out float dist) {
    vec2 seg = line_1 - line_0;
    vec2 seg_perp = vec2(seg.y, -seg.x);
    float pd = dot(ray_direction, seg_perp);

    if (abs(pd) <= 0.0) {
        dist = 100000000.0;
        return false;
    }

    vec2 d = line_0 - ray_origin;

    dist = dot(seg_perp, d) / pd;
    float s = dot(vec2(ray_direction.y, -ray_direction.x), d) / pd;

    return dist >= 0.0 && s >= 0.0 && s <= 1.0 && dist < max_dist;
}

vec2 ray_polygon(vec2 ray_origin, vec2 ray_direction, int start, int count, float max_dist) {
    uint crossings = 0;

    float dist = 2000000000.0;
    float t = dist;

    for (int i = start; i < start + count; i++) {
        int n = (i + 1);

        if (n >= start + count) {
            n = start;
        }

        if (ray_line(ray_origin, ray_direction, object_position[i], object_position[n], max_dist, dist)) {
            crossings++;

            if (dist < t) {
                t = dist;
            }
        }
    }

    if (crossings > 0 && crossings % 2 == 0) {
        return vec2(1, t);
    } else if (crossings > 0) {
        return vec2(2, t);
    } else {
        return vec2(0, t);
    }
}

vec3 apply_shadows(vec3 c) {
    vec2 pos = (2.0 * gl_FragCoord.xy-resolution)/resolution.y * camera_size + camera_position;

    bool lit = false;
 
    vec3 col = vec3(0.0);

    for (int i = 0; i < light_position.length(); i++) {
        float d = length(light_position[i]-pos);

        int start = 0;
        bool hit_object = false;

        vec2 ray_direction = normalize(light_position[i] - pos);

        for (int j = 0; j < object_length.length(); j++) {
            int len = object_length[j];

            //if (len != 20) {
            //    return vec4(0.2, 0.5, 0.2, 1.0);
            //}

            vec2 hit = ray_polygon(pos, ray_direction, start, len, d);

            start += len;

            if (hit.x == 1) {
                hit_object = true;
            } else if (hit.x == 2) {
                hit_object = true;
                lit = true;

                col += (0.01-clamp(smoothstep(0.0, 1.0, hit.y * pow(d*1, 1.5) / lights[i].x * 10.0), 0.0, 1.0)) * lights[i].yzw * c;
            }
        }

        if (!hit_object) {
            lit = true;

            col += lights[i].yzw * c / (pow(d, 2.0)/lights[i].x*20.0);
        }
    }

    if (lit) {
        return col;
    } else {
        return vec3(0.0, 0.0, 0.0);
    }
}

void main() {
    vec2 pos = (gl_FragCoord.xy/resolution) * 2.0 - 1.0;

    color = vec4(apply_shadows(vec3(1.0)), 1.0);
}
