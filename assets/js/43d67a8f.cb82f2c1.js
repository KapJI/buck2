"use strict";(self.webpackChunkwebsite=self.webpackChunkwebsite||[]).push([[9779],{3905:(e,t,n)=>{n.r(t),n.d(t,{MDXContext:()=>c,MDXProvider:()=>p,mdx:()=>b,useMDXComponents:()=>u,withMDXComponents:()=>d});var r=n(67294);function a(e,t,n){return t in e?Object.defineProperty(e,t,{value:n,enumerable:!0,configurable:!0,writable:!0}):e[t]=n,e}function i(){return i=Object.assign||function(e){for(var t=1;t<arguments.length;t++){var n=arguments[t];for(var r in n)Object.prototype.hasOwnProperty.call(n,r)&&(e[r]=n[r])}return e},i.apply(this,arguments)}function o(e,t){var n=Object.keys(e);if(Object.getOwnPropertySymbols){var r=Object.getOwnPropertySymbols(e);t&&(r=r.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),n.push.apply(n,r)}return n}function s(e){for(var t=1;t<arguments.length;t++){var n=null!=arguments[t]?arguments[t]:{};t%2?o(Object(n),!0).forEach((function(t){a(e,t,n[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(n)):o(Object(n)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(n,t))}))}return e}function l(e,t){if(null==e)return{};var n,r,a=function(e,t){if(null==e)return{};var n,r,a={},i=Object.keys(e);for(r=0;r<i.length;r++)n=i[r],t.indexOf(n)>=0||(a[n]=e[n]);return a}(e,t);if(Object.getOwnPropertySymbols){var i=Object.getOwnPropertySymbols(e);for(r=0;r<i.length;r++)n=i[r],t.indexOf(n)>=0||Object.prototype.propertyIsEnumerable.call(e,n)&&(a[n]=e[n])}return a}var c=r.createContext({}),d=function(e){return function(t){var n=u(t.components);return r.createElement(e,i({},t,{components:n}))}},u=function(e){var t=r.useContext(c),n=t;return e&&(n="function"==typeof e?e(t):s(s({},t),e)),n},p=function(e){var t=u(e.components);return r.createElement(c.Provider,{value:t},e.children)},m="mdxType",h={inlineCode:"code",wrapper:function(e){var t=e.children;return r.createElement(r.Fragment,{},t)}},f=r.forwardRef((function(e,t){var n=e.components,a=e.mdxType,i=e.originalType,o=e.parentName,c=l(e,["components","mdxType","originalType","parentName"]),d=u(n),p=a,m=d["".concat(o,".").concat(p)]||d[p]||h[p]||i;return n?r.createElement(m,s(s({ref:t},c),{},{components:n})):r.createElement(m,s({ref:t},c))}));function b(e,t){var n=arguments,a=t&&t.mdxType;if("string"==typeof e||a){var i=n.length,o=new Array(i);o[0]=f;var s={};for(var l in t)hasOwnProperty.call(t,l)&&(s[l]=t[l]);s.originalType=e,s[m]="string"==typeof e?e:a,o[1]=s;for(var c=2;c<i;c++)o[c]=n[c];return r.createElement.apply(null,o)}return r.createElement.apply(null,n)}f.displayName="MDXCreateElement"},72307:(e,t,n)=>{n.r(t),n.d(t,{assets:()=>l,contentTitle:()=>o,default:()=>u,frontMatter:()=>i,metadata:()=>s,toc:()=>c});var r=n(87462),a=(n(67294),n(3905));const i={id:"bxl_faqs",title:"FAQs"},o=void 0,s={unversionedId:"developers/bxl_faqs",id:"developers/bxl_faqs",title:"FAQs",description:"When should I use BXL over Buck2 API/CLI?",source:"@site/../docs/developers/bxl_faq.md",sourceDirName:"developers",slug:"/developers/bxl_faqs",permalink:"/docs/developers/bxl_faqs",draft:!1,tags:[],version:"current",frontMatter:{id:"bxl_faqs",title:"FAQs"},sidebar:"manualSidebar",previous:{title:"BXL and Anonymous Targets",permalink:"/docs/developers/anon_targets"},next:{title:"action type",permalink:"/docs/api/bxl/action"}},l={},c=[{value:"When should I use BXL over Buck2 API/CLI?",id:"when-should-i-use-bxl-over-buck2-apicli",level:2},{value:"When is my BXL script cached?",id:"when-is-my-bxl-script-cached",level:2},{value:"What\u2019s the difference between <code>ctx.output.print()</code> and <code>print()</code>?",id:"whats-the-difference-between-ctxoutputprint-and-print",level:2},{value:"What do I need to know about ensured artifacts",id:"what-do-i-need-to-know-about-ensured-artifacts",level:2}],d={toc:c};function u(e){let{components:t,...n}=e;return(0,a.mdx)("wrapper",(0,r.Z)({},d,n,{components:t,mdxType:"MDXLayout"}),(0,a.mdx)("h2",{id:"when-should-i-use-bxl-over-buck2-apicli"},"When should I use BXL over Buck2 API/CLI?"),(0,a.mdx)("p",null,"There are many overlaps between BXL and Buck2 (for example, both can run cquery and both can build targets). It\u2019s possible that one use case could be handled by both BXL and Buck2."),(0,a.mdx)("p",null,"Following are some specific recommendations to help decide when to use BXL over regular Buck2:"),(0,a.mdx)("ul",null,(0,a.mdx)("li",{parentName:"ul"},(0,a.mdx)("strong",{parentName:"li"},"Use/inspect resolved attributes that are not exposed/accessible to users via normal Buck2 operations."),(0,a.mdx)("ul",{parentName:"li"},(0,a.mdx)("li",{parentName:"ul"},"This includes introspecting the Starlark object of providers, analyzing the Starlark object of a rule\u2019s attr before and after coercing and resolution, and introspecting intermediate query results."))),(0,a.mdx)("li",{parentName:"ul"},(0,a.mdx)("strong",{parentName:"li"},"Reduce/eliminate the need to make several Buck2 calls within your program, such as running several subprocesses to call ",(0,a.mdx)("inlineCode",{parentName:"strong"},"cquery")," several times."),(0,a.mdx)("ul",{parentName:"li"},(0,a.mdx)("li",{parentName:"ul"},"With BXL, you can just call the BXL script once in a subprocess, potentially reducing the amount of code you need to write in your program."))),(0,a.mdx)("li",{parentName:"ul"},(0,a.mdx)("strong",{parentName:"li"},"Reduce/eliminate the need to manually parse Buck2 output format within your program, and any bugs that may come with manual parsing"),".",(0,a.mdx)("ul",{parentName:"li"},(0,a.mdx)("li",{parentName:"ul"},"Some languages are more verbose than others when it comes to string parsing."),(0,a.mdx)("li",{parentName:"ul"},"BXL scripts are written in Starlark, which is basically a deterministic, immutable Python, and are able to directly introspect Starlark objects (such as rules and target nodes, and so on) and call methods on these objects instead of parsing them over Buck2\u2019s output.")))),(0,a.mdx)("h2",{id:"when-is-my-bxl-script-cached"},"When is my BXL script cached?"),(0,a.mdx)("p",null,"The entire BXL script is represented as a single node on the DICE graph (Buck2\u2019s internal dependency graph). When the script\u2019s input changes, the entire node is invalidated and needs to be recomputed. For example, if a BXL function calls uquery, then uses the result to do a cquery and then a build, if Buck2 detects that any of the recorded calls to uquery, cquery, and build changes, then the entire BXL script will be reran. The computations themselves (uquery, cquery, and build) will still be incrementally evaluated via DICE, so we are not rerunning ",(0,a.mdx)("em",{parentName:"p"},"every")," computation entirely within the BXL."),(0,a.mdx)("p",null,"When the BXL script creates artifacts and ensures them, those artifacts are cached separately in an action outside of the BXL execution. This means that the artifacts produced by BXL are cached separately from the BXL script itself, much like the computations within a BXL."),(0,a.mdx)("p",null,"During 2023, there is a plan to add finer grain incrementality to make better use of DICE\u2019s existing incrementality support."),(0,a.mdx)("h2",{id:"whats-the-difference-between-ctxoutputprint-and-print"},"What\u2019s the difference between ",(0,a.mdx)("inlineCode",{parentName:"h2"},"ctx.output.print()")," and ",(0,a.mdx)("inlineCode",{parentName:"h2"},"print()"),"?"),(0,a.mdx)("ul",null,(0,a.mdx)("li",{parentName:"ul"},(0,a.mdx)("inlineCode",{parentName:"li"},"ctx.output.print()")," writes items to stdout by buck2 even when the script is cached. Items written to the output stream are considered to be the results of a BXL script, which will be displayed to stdout by buck2 even when the script is cached."),(0,a.mdx)("li",{parentName:"ul"},(0,a.mdx)("inlineCode",{parentName:"li"},"print()")," is offered by Starlark via the stdlib. This prints anything you want but won\u2019t be provided to stdout at the end of a BXL script. These can be used to print to stderr. NOTE: ",(0,a.mdx)("inlineCode",{parentName:"li"},"print()")," statements don't show up if the script has been cached.")),(0,a.mdx)("h2",{id:"what-do-i-need-to-know-about-ensured-artifacts"},"What do I need to know about ensured artifacts"),(0,a.mdx)("p",null,"An ",(0,a.mdx)("inlineCode",{parentName:"p"},"ensured_artifact")," prints out the relative or absolute path via ",(0,a.mdx)("inlineCode",{parentName:"p"},"ctx.output.print()"),", depending on if called with ",(0,a.mdx)("inlineCode",{parentName:"p"},"abs_path()")," or ",(0,a.mdx)("inlineCode",{parentName:"p"},"rel_path"),"(), but will print out ",(0,a.mdx)("inlineCode",{parentName:"p"},"<ensured artifact bound to <some path>>")," via ",(0,a.mdx)("inlineCode",{parentName:"p"},"print()"),"."),(0,a.mdx)("p",null,"This is intentional because when the ensured artifact is created with BXL within BXL, it has not been materialized yet. It will be materialized after the BXL script finishes executing, and Buck2 core performs some additional actions after the BXL script."),(0,a.mdx)("p",null,"This is a safeguard to prevent people from misusing the artifact path and passing it into an action without the artifact having been materialized or passing an absolute path into RE, which can actually mess up RE and render the action not shareable across users. In addition, it makes these actions separately cacheable from the BXL execution."))}u.isMDXComponent=!0}}]);