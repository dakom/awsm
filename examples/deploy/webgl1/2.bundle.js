(window.webpackJsonp=window.webpackJsonp||[]).push([[2],[,,function(n,t,r){var e=r(4);"string"==typeof e&&(e=[[n.i,e,""]]);var o={hmr:!0,transform:void 0,insertInto:void 0};r(6)(e,o);e.locals&&(n.exports=e.locals)},,function(n,t,r){(n.exports=r(5)(!1)).push([n.i,"/* Global */\r\nhtml {\r\n  box-sizing: border-box;\r\n}\r\n*, *:before, *:after {\r\n  box-sizing: inherit;\r\n}\r\n\r\nhtml,body {\r\n\tpadding: 0;\r\n\tmargin: 0;\r\n\tfont-family: Arial, Helvetica, sans-serif;\r\n}\r\n\r\ncanvas {\r\n    position: absolute;\r\n    top: 0;\r\n    left: 0;\r\n    padding: 0;\r\n    margin: 0;\r\n}\r\n\r\na {\r\n\ttext-decoration: none;\r\n}\r\n\r\nul {\r\n\tlist-style: none;\t\r\n}\r\n\r\n\r\n.demo-links{\r\n    z-index: 1;\r\n    display: flex;\r\n    flex-direction: row;\r\n    position: absolute;\r\n    top: 0px;\r\n    right: 0px;\r\n}\r\n.source, .home {\r\n    display: inline-block;\r\n}\r\n\r\n.button {\r\n\tcursor: pointer;\r\n\tmargin: 10px;\r\n\tbackground-color: #0583F2;\r\n\tcolor: white;\r\n\ttext-align: center;\r\n\tpadding: 1em 2em;\r\n\tborder-radius: 5px;\r\n\tfont-weight: bold;\r\n}\r\n\r\n.button:hover {\r\n\tbackground-color: #F27405;\r\n}\r\n\r\n/* color values for theme \r\n#BC04BF\r\n#0583F2\r\n#0ABF04\r\n#F2CB05\r\n#F27405\r\n*/\r\n\r\n/* Menu */\r\n.menu-header {\r\n\tmargin-top: 5px;\r\n\tmargin-left: 10px;\r\n\tfont-size: 2rem;\r\n}\r\n.menu-list {\r\n\tdisplay: flex;\r\n\tflex-direction: row;\r\n}\r\n\r\n\r\n/* scenes */\r\n.tick, .loaders {\r\n    padding: 20px;\r\n}\r\n",""])},function(n,t,r){"use strict";n.exports=function(n){var t=[];return t.toString=function(){return this.map(function(t){var r=function(n,t){var r=n[1]||"",e=n[3];if(!e)return r;if(t&&"function"==typeof btoa){var o=(s=e,"/*# sourceMappingURL=data:application/json;charset=utf-8;base64,"+btoa(unescape(encodeURIComponent(JSON.stringify(s))))+" */"),i=e.sources.map(function(n){return"/*# sourceURL="+e.sourceRoot+n+" */"});return[r].concat(i).concat([o]).join("\n")}var s;return[r].join("\n")}(t,n);return t[2]?"@media "+t[2]+"{"+r+"}":r}).join("")},t.i=function(n,r){"string"==typeof n&&(n=[[null,n,""]]);for(var e={},o=0;o<this.length;o++){var i=this[o][0];null!=i&&(e[i]=!0)}for(o=0;o<n.length;o++){var s=n[o];null!=s[0]&&e[s[0]]||(r&&!s[2]?s[2]=r:r&&(s[2]="("+s[2]+") and ("+r+")"),t.push(s))}},t}},function(n,t,r){var e,o,i={},s=(e=function(){return window&&document&&document.all&&!window.atob},function(){return void 0===o&&(o=e.apply(this,arguments)),o}),a=function(n){var t={};return function(n,r){if("function"==typeof n)return n();if(void 0===t[n]){var e=function(n,t){return t?t.querySelector(n):document.querySelector(n)}.call(this,n,r);if(window.HTMLIFrameElement&&e instanceof window.HTMLIFrameElement)try{e=e.contentDocument.head}catch(n){e=null}t[n]=e}return t[n]}}(),c=null,f=0,u=[],l=r(7);function p(n,t){for(var r=0;r<n.length;r++){var e=n[r],o=i[e.id];if(o){o.refs++;for(var s=0;s<o.parts.length;s++)o.parts[s](e.parts[s]);for(;s<e.parts.length;s++)o.parts.push(y(e.parts[s],t))}else{var a=[];for(s=0;s<e.parts.length;s++)a.push(y(e.parts[s],t));i[e.id]={id:e.id,refs:1,parts:a}}}}function d(n,t){for(var r=[],e={},o=0;o<n.length;o++){var i=n[o],s=t.base?i[0]+t.base:i[0],a={css:i[1],media:i[2],sourceMap:i[3]};e[s]?e[s].parts.push(a):r.push(e[s]={id:s,parts:[a]})}return r}function h(n,t){var r=a(n.insertInto);if(!r)throw new Error("Couldn't find a style target. This probably means that the value for the 'insertInto' parameter is invalid.");var e=u[u.length-1];if("top"===n.insertAt)e?e.nextSibling?r.insertBefore(t,e.nextSibling):r.appendChild(t):r.insertBefore(t,r.firstChild),u.push(t);else if("bottom"===n.insertAt)r.appendChild(t);else{if("object"!=typeof n.insertAt||!n.insertAt.before)throw new Error("[Style Loader]\n\n Invalid value for parameter 'insertAt' ('options.insertAt') found.\n Must be 'top', 'bottom', or Object.\n (https://github.com/webpack-contrib/style-loader#insertat)\n");var o=a(n.insertAt.before,r);r.insertBefore(t,o)}}function v(n){if(null===n.parentNode)return!1;n.parentNode.removeChild(n);var t=u.indexOf(n);t>=0&&u.splice(t,1)}function b(n){var t=document.createElement("style");if(void 0===n.attrs.type&&(n.attrs.type="text/css"),void 0===n.attrs.nonce){var e=function(){0;return r.nc}();e&&(n.attrs.nonce=e)}return m(t,n.attrs),h(n,t),t}function m(n,t){Object.keys(t).forEach(function(r){n.setAttribute(r,t[r])})}function y(n,t){var r,e,o,i;if(t.transform&&n.css){if(!(i="function"==typeof t.transform?t.transform(n.css):t.transform.default(n.css)))return function(){};n.css=i}if(t.singleton){var s=f++;r=c||(c=b(t)),e=x.bind(null,r,s,!1),o=x.bind(null,r,s,!0)}else n.sourceMap&&"function"==typeof URL&&"function"==typeof URL.createObjectURL&&"function"==typeof URL.revokeObjectURL&&"function"==typeof Blob&&"function"==typeof btoa?(r=function(n){var t=document.createElement("link");return void 0===n.attrs.type&&(n.attrs.type="text/css"),n.attrs.rel="stylesheet",m(t,n.attrs),h(n,t),t}(t),e=function(n,t,r){var e=r.css,o=r.sourceMap,i=void 0===t.convertToAbsoluteUrls&&o;(t.convertToAbsoluteUrls||i)&&(e=l(e));o&&(e+="\n/*# sourceMappingURL=data:application/json;base64,"+btoa(unescape(encodeURIComponent(JSON.stringify(o))))+" */");var s=new Blob([e],{type:"text/css"}),a=n.href;n.href=URL.createObjectURL(s),a&&URL.revokeObjectURL(a)}.bind(null,r,t),o=function(){v(r),r.href&&URL.revokeObjectURL(r.href)}):(r=b(t),e=function(n,t){var r=t.css,e=t.media;e&&n.setAttribute("media",e);if(n.styleSheet)n.styleSheet.cssText=r;else{for(;n.firstChild;)n.removeChild(n.firstChild);n.appendChild(document.createTextNode(r))}}.bind(null,r),o=function(){v(r)});return e(n),function(t){if(t){if(t.css===n.css&&t.media===n.media&&t.sourceMap===n.sourceMap)return;e(n=t)}else o()}}n.exports=function(n,t){if("undefined"!=typeof DEBUG&&DEBUG&&"object"!=typeof document)throw new Error("The style-loader cannot be used in a non-browser environment");(t=t||{}).attrs="object"==typeof t.attrs?t.attrs:{},t.singleton||"boolean"==typeof t.singleton||(t.singleton=s()),t.insertInto||(t.insertInto="head"),t.insertAt||(t.insertAt="bottom");var r=d(n,t);return p(r,t),function(n){for(var e=[],o=0;o<r.length;o++){var s=r[o];(a=i[s.id]).refs--,e.push(a)}n&&p(d(n,t),t);for(o=0;o<e.length;o++){var a;if(0===(a=e[o]).refs){for(var c=0;c<a.parts.length;c++)a.parts[c]();delete i[a.id]}}}};var g,w=(g=[],function(n,t){return g[n]=t,g.filter(Boolean).join("\n")});function x(n,t,r,e){var o=r?"":e.css;if(n.styleSheet)n.styleSheet.cssText=w(t,o);else{var i=document.createTextNode(o),s=n.childNodes;s[t]&&n.removeChild(s[t]),s.length?n.insertBefore(i,s[t]):n.appendChild(i)}}},function(n,t){n.exports=function(n){var t="undefined"!=typeof window&&window.location;if(!t)throw new Error("fixUrls requires window.location");if(!n||"string"!=typeof n)return n;var r=t.protocol+"//"+t.host,e=r+t.pathname.replace(/\/[^\/]*$/,"/");return n.replace(/url\s*\(((?:[^)(]|\((?:[^)(]+|\([^)(]*\))*\))*)\)/gi,function(n,t){var o,i=t.trim().replace(/^"(.*)"$/,function(n,t){return t}).replace(/^'(.*)'$/,function(n,t){return t});return/^(#|data:|http:\/\/|https:\/\/|file:\/\/\/|\s*$)/i.test(i)?n:(o=0===i.indexOf("//")?i:0===i.indexOf("/")?r+i:e+i.replace(/^\.\//,""),"url("+JSON.stringify(o)+")")})}}]]);