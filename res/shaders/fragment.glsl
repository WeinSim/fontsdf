#version 460 core

in vec2 uv;

out vec4 outColor;

// uniform int method;
const int method = 0;

// const vec2 point = vec2(0.4, 0.6);

// source: https://www.shadertoy.com/view/MlKcDD

float dot2( vec2 v ) { return dot(v,v); }
float cro( vec2 a, vec2 b ) { return a.x*b.y-a.y*b.x; }
float cos_acos_3( float x ) { x=sqrt(0.5+0.5*x); return x*(x*(x*(x*-0.008972+0.039071)-0.107074)+0.576975)+0.5; } // https://www.shadertoy.com/view/WltSD7

// signed distance to a quadratic bezier
float sdBezier( in vec2 pos, in vec2 A, in vec2 B, in vec2 C, out vec2 outQ )
{    
    vec2 a = B - A;
    vec2 b = A - 2.0*B + C;
    vec2 c = a * 2.0;
    vec2 d = A - pos;

    // cubic to be solved (kx*=3 and ky*=3)
    float kk = 1.0/dot(b,b);
    float kx = kk * dot(a,b);
    float ky = kk * (2.0*dot(a,a)+dot(d,b))/3.0;
    float kz = kk * dot(d,a);      

    float res = 0.0;
    float sgn = 0.0;

    float p  = ky - kx*kx;
    float q  = kx*(2.0*kx*kx - 3.0*ky) + kz;
    float p3 = p*p*p;
    float q2 = q*q;
    float h  = q2 + 4.0*p3;


    if( h>=0.0 ) 
    {   // 1 root
        h = sqrt(h);
        
        #if 0
            vec2 x = (vec2(h,-h)-q)/2.0;
            #if 0
            // When p≈0 and p<0, h-q has catastrophic cancelation. So, we do
            // h=√(q²+4p³)=q·√(1+4p³/q²)=q·√(1+w) instead. Now we approximate
            // √ by a linear Taylor expansion into h≈q(1+½w) so that the q's
            // cancel each other in h-q. Expanding and simplifying further we
            // get x=vec2(p³/q,-p³/q-q). And using a second degree Taylor
            // expansion instead: x=vec2(k,-k-q) with k=(1-p³/q²)·p³/q
            if( abs(p)<0.001 )
            {
                float k = p3/q;              // linear approx
              //float k = (1.0-p3/q2)*p3/q;  // quadratic approx 
                x = vec2(k,-k-q);  
            }
            #endif

            vec2 uv = sign(x)*pow(abs(x), vec2(1.0/3.0));
            float t = uv.x + uv.y;
        #else
            h = (q<0.0) ? h : -h; // copysign()
            float x = (h-q)/2.0;
            float v = sign(x)*pow(abs(x),1.0/3.0);
            float t = v - p/v;
        #endif

		// from NinjaKoala - single newton iteration to account for cancellation
        t -= (t*(t*t+3.0*p)+q)/(3.0*t*t+3.0*p);
        
        t = clamp( t-kx, 0.0, 1.0 );
        vec2  w = d+(c+b*t)*t;
        outQ = w + pos;
        res = dot2(w);
    	sgn = cro(c+2.0*b*t,w);
    }
    else 
    {   // 3 roots
        float z = sqrt(-p);
        #if 0
        float v = acos(q/(p*z*2.0))/3.0;
        float m = cos(v);
        float n = sin(v);
        #else
        float m = cos_acos_3( q/(p*z*2.0) );
        float n = sqrt(1.0-m*m);
        #endif
        n *= sqrt(3.0);
        vec3  t = clamp( vec3(m+m,-n-m,n-m)*z-kx, 0.0, 1.0 );
        vec2  qx=d+(c+b*t.x)*t.x; float dx=dot2(qx), sx=cro(a+b*t.x,qx);
        vec2  qy=d+(c+b*t.y)*t.y; float dy=dot2(qy), sy=cro(a+b*t.y,qy);
        if( dx<dy ) {res=dx;sgn=sx;outQ=qx+pos;} else {res=dy;sgn=sy;outQ=qy+pos;}
    }
    
    return sqrt( res )*sign(sgn);
}

void main(void) {
    vec2 A = vec2(0, 0);
    vec2 B = vec2(2.5, 0.5);
    vec2 C = vec2(0, 1);
    vec2 outQ = vec2(0, 0);
    float dist;
    if (method == 0) {
        dist = sdBezier(uv, A, B, C, outQ);
    } else {
        dist = distance(uv, vec2(0.5, 0.5)) - 0.3;
    }
    if (abs(dist) < 0.001) {
        outColor = vec4(1, 1, 1, 1);
    } else {
        // float dmin = -0.5;
        // float dmax = 1;
        // float d = (dist - dmin) / (dmax - dmin);
        // outColor = vec4(vec3(1, 1, 1) * d, 1);
        outColor = vec4(vec3(1, 0, -1) * dist, 1);
    }
}
