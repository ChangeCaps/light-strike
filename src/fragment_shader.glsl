#version 450

out vec4 color;

buffer object_lengths {
    int object_length[];
};

buffer object_positions {
    vec2 object_position[];
};

buffer light_positions {
    vec2 light_position[];
};

buffer light_strengths {
    float light_strength[];
};

uniform vec2 resolution;


bool ray_line(vec2 ray_origin, vec2 ray_direction, vec2 line_0, vec2 line_1, float max_dist) {
    vec2 seg = line_1 - line_0;
    vec2 seg_perp = vec2(seg.y, -seg.x);
    float pd = dot(ray_direction, seg_perp);

    if (abs(pd) <= 0.0) {
        return false;
    }

    vec2 d = line_0 - ray_origin;

    float dist = dot(seg_perp, d) / pd;
    float s = dot(vec2(ray_direction.y, -ray_direction.x), d) / pd;

    return dist >= 0.0 && s >= 0.0 && s <= 1.0 && dist < max_dist;
}

int ray_polygon(vec2 ray_origin, vec2 ray_direction, int start, int count, float max_dist) {
    uint crossings = 0;

    for (int i = start; i < start + count; i++) {
        int n = (i + 1);

        if (n >= start + count) {
            n = start;
        }

        if (ray_line(ray_origin, ray_direction, object_position[i], object_position[n], max_dist)) {
            crossings++;
        }
    }

    if (crossings > 0) {
        return 1;
    } else {
        return 0;
    }
}

vec4 apply_shadows(vec4 c) {
    vec2 pos = (gl_FragCoord.xy/resolution) * 2.0 - 1.0;

    float dist = -1.0;
    bool lit = false;

    for (int i = 0; i < light_position.length(); i++) {
        float d = length(light_position[i]-pos);

        int start = 0;
        bool hit_object = false;

        vec2 ray_direction = normalize(light_position[i]-pos);

        for (int j = 0; j < object_length.length(); j++) {
            int len = object_length[j];

            //if (len != 20) {
            //    return vec4(0.2, 0.5, 0.2, 1.0);
            //}

            int hit = ray_polygon(pos, ray_direction, start, len, d);

            start += len;

            if (hit == 1) {
                hit_object = true;
            }
        }

        if (!hit_object) {
            lit = true;

            if (d < dist || dist < 0.0) {
                dist = d;
            }
        }
    }

    if (lit) {
        return c / (pow(dist*20, 2));
    } else {
        return vec4(0.0, 0.0, 0.0, 1.0);
    }
}

void main() {
    vec2 pos = (gl_FragCoord.xy/resolution) * 2.0 - 1.0;

    color = apply_shadows(vec4(1.0));
}
