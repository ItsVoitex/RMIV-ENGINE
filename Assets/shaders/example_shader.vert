#version 330 core
        layout (location = 0) in vec3 aPos;
        layout (location = 1) in vec2 aTexCoord;
        layout (location = 2) in vec3 aColor;
        out vec2 texCoord;
        out vec3 Color;
        uniform vec3 offset;
        uniform vec3 rotation;
        uniform vec3 scale;

        uniform mat4 model;
        uniform mat4 view;
        uniform mat4 proj;

        void main()
        {
            float angle = radians(rotation.x);
            float s = sin(angle);
            float c = cos(angle);
            vec3 position = aPos * scale;
            position.xy = mat2(c,-s,s,c) *position.xy;
            position = position + offset;
            position.xy /= 540;
            
            gl_Position = proj * view * model vec4(position, 1.0);
            texCoord = aTexCoord;
            Color = aColor; 
        }