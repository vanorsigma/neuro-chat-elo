function x(){}const A=t=>t;function w(t,n){for(const e in n)t[e]=n[e];return t}function j(t){return t()}function D(){return Object.create(null)}function E(t){t.forEach(j)}function F(t){return typeof t=="function"}function P(t,n){return t!=t?n==n:t!==n||t&&typeof t=="object"||typeof t=="function"}let i;function S(t,n){return t===n?!0:(i||(i=document.createElement("a")),i.href=n,t===i.href)}function U(t){return Object.keys(t).length===0}function q(t,...n){if(t==null){for(const o of n)o(void 0);return x}const e=t.subscribe(...n);return e.unsubscribe?()=>e.unsubscribe():e}function B(t,n,e){t.$$.on_destroy.push(q(n,e))}function C(t,n,e,o){if(t){const r=m(t,n,e,o);return t[0](r)}}function m(t,n,e,o){return t[1]&&o?w(e.ctx.slice(),t[1](o(n))):e.ctx}function G(t,n,e,o){if(t[2]&&o){const r=t[2](o(e));if(n.dirty===void 0)return r;if(typeof r=="object"){const l=[],_=Math.max(n.dirty.length,r.length);for(let s=0;s<_;s+=1)l[s]=n.dirty[s]|r[s];return l}return n.dirty|r}return n.dirty}function H(t,n,e,o,r,l){if(r){const _=m(n,e,o,l);t.p(_,r)}}function I(t){if(t.ctx.length>32){const n=[],e=t.ctx.length/32;for(let o=0;o<e;o++)n[o]=-1;return n}return-1}let f;function d(t){f=t}function g(){if(!f)throw new Error("Function called outside component initialization");return f}function J(t){g().$$.on_mount.push(t)}function K(t){g().$$.after_update.push(t)}function L(t){g().$$.on_destroy.push(t)}const a=[],y=[];let u=[];const p=[],k=Promise.resolve();let b=!1;function v(){b||(b=!0,k.then(z))}function N(){return v(),k}function O(t){u.push(t)}function Q(t){p.push(t)}const h=new Set;let c=0;function z(){if(c!==0)return;const t=f;do{try{for(;c<a.length;){const n=a[c];c++,d(n),M(n.$$)}}catch(n){throw a.length=0,c=0,n}for(d(null),a.length=0,c=0;y.length;)y.pop()();for(let n=0;n<u.length;n+=1){const e=u[n];h.has(e)||(h.add(e),e())}u.length=0}while(a.length);for(;p.length;)p.pop()();b=!1,h.clear(),d(t)}function M(t){if(t.fragment!==null){t.update(),E(t.before_update);const n=t.dirty;t.dirty=[-1],t.fragment&&t.fragment.p(t.ctx,n),t.after_update.forEach(O)}}function R(t){const n=[],e=[];u.forEach(o=>t.indexOf(o)===-1?n.push(o):e.push(o)),e.forEach(o=>o()),u=n}export{Q as A,C as a,G as b,B as c,K as d,y as e,O as f,I as g,A as h,F as i,D as j,z as k,U as l,R as m,x as n,J as o,f as p,d as q,E as r,P as s,N as t,H as u,j as v,a as w,v as x,L as y,S as z};
