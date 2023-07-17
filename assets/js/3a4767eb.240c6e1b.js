"use strict";(self.webpackChunkwebsite=self.webpackChunkwebsite||[]).push([[6724],{3905:(e,t,a)=>{a.r(t),a.d(t,{MDXContext:()=>o,MDXProvider:()=>m,mdx:()=>x,useMDXComponents:()=>u,withMDXComponents:()=>p});var n=a(67294);function r(e,t,a){return t in e?Object.defineProperty(e,t,{value:a,enumerable:!0,configurable:!0,writable:!0}):e[t]=a,e}function l(){return l=Object.assign||function(e){for(var t=1;t<arguments.length;t++){var a=arguments[t];for(var n in a)Object.prototype.hasOwnProperty.call(a,n)&&(e[n]=a[n])}return e},l.apply(this,arguments)}function s(e,t){var a=Object.keys(e);if(Object.getOwnPropertySymbols){var n=Object.getOwnPropertySymbols(e);t&&(n=n.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),a.push.apply(a,n)}return a}function d(e){for(var t=1;t<arguments.length;t++){var a=null!=arguments[t]?arguments[t]:{};t%2?s(Object(a),!0).forEach((function(t){r(e,t,a[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(a)):s(Object(a)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(a,t))}))}return e}function i(e,t){if(null==e)return{};var a,n,r=function(e,t){if(null==e)return{};var a,n,r={},l=Object.keys(e);for(n=0;n<l.length;n++)a=l[n],t.indexOf(a)>=0||(r[a]=e[a]);return r}(e,t);if(Object.getOwnPropertySymbols){var l=Object.getOwnPropertySymbols(e);for(n=0;n<l.length;n++)a=l[n],t.indexOf(a)>=0||Object.prototype.propertyIsEnumerable.call(e,a)&&(r[a]=e[a])}return r}var o=n.createContext({}),p=function(e){return function(t){var a=u(t.components);return n.createElement(e,l({},t,{components:a}))}},u=function(e){var t=n.useContext(o),a=t;return e&&(a="function"==typeof e?e(t):d(d({},t),e)),a},m=function(e){var t=u(e.components);return n.createElement(o.Provider,{value:t},e.children)},c="mdxType",h={inlineCode:"code",wrapper:function(e){var t=e.children;return n.createElement(n.Fragment,{},t)}},f=n.forwardRef((function(e,t){var a=e.components,r=e.mdxType,l=e.originalType,s=e.parentName,o=i(e,["components","mdxType","originalType","parentName"]),p=u(a),m=r,c=p["".concat(s,".").concat(m)]||p[m]||h[m]||l;return a?n.createElement(c,d(d({ref:t},o),{},{components:a})):n.createElement(c,d({ref:t},o))}));function x(e,t){var a=arguments,r=t&&t.mdxType;if("string"==typeof e||r){var l=a.length,s=new Array(l);s[0]=f;var d={};for(var i in t)hasOwnProperty.call(t,i)&&(d[i]=t[i]);d.originalType=e,d[c]="string"==typeof e?e:r,s[1]=d;for(var o=2;o<l;o++)s[o]=a[o];return n.createElement.apply(null,s)}return n.createElement.apply(null,a)}f.displayName="MDXCreateElement"},65319:(e,t,a)=>{a.r(t),a.d(t,{assets:()=>i,contentTitle:()=>s,default:()=>u,frontMatter:()=>l,metadata:()=>d,toc:()=>o});var n=a(87462),r=(a(67294),a(3905));const l={id:"attrs"},s="attrs type",d={unversionedId:"api/build/attrs",id:"api/build/attrs",title:"attrs type",description:"This type is available as a global attrs symbol, to allow the definition of attributes to the rule function.",source:"@site/../docs/api/build/attrs.generated.md",sourceDirName:"api/build",slug:"/api/build/attrs",permalink:"/docs/api/build/attrs",draft:!1,tags:[],version:"current",frontMatter:{id:"attrs"},sidebar:"manualSidebar",previous:{title:"artifact type",permalink:"/docs/api/build/artifact"},next:{title:"cmd_args type",permalink:"/docs/api/build/cmd_args"}},i={},o=[{value:"attrs.any",id:"attrsany",level:2},{value:"attrs.arg",id:"attrsarg",level:2},{value:"attrs.bool",id:"attrsbool",level:2},{value:"attrs.configuration_label",id:"attrsconfiguration_label",level:2},{value:"attrs.configured_dep",id:"attrsconfigured_dep",level:2},{value:"attrs.default_only",id:"attrsdefault_only",level:2},{value:"attrs.dep",id:"attrsdep",level:2},{value:"attrs.dict",id:"attrsdict",level:2},{value:"attrs.enum",id:"attrsenum",level:2},{value:"attrs.exec_dep",id:"attrsexec_dep",level:2},{value:"attrs.int",id:"attrsint",level:2},{value:"attrs.label",id:"attrslabel",level:2},{value:"attrs.list",id:"attrslist",level:2},{value:"attrs.named_set",id:"attrsnamed_set",level:2},{value:"attrs.one_of",id:"attrsone_of",level:2},{value:"attrs.option",id:"attrsoption",level:2},{value:"attrs.query",id:"attrsquery",level:2},{value:"attrs.regex",id:"attrsregex",level:2},{value:"attrs.set",id:"attrsset",level:2},{value:"attrs.source",id:"attrssource",level:2},{value:"attrs.split_transition_dep",id:"attrssplit_transition_dep",level:2},{value:"attrs.string",id:"attrsstring",level:2},{value:"attrs.toolchain_dep",id:"attrstoolchain_dep",level:2},{value:"attrs.transition_dep",id:"attrstransition_dep",level:2},{value:"attrs.tuple",id:"attrstuple",level:2},{value:"attrs.versioned",id:"attrsversioned",level:2}],p={toc:o};function u(e){let{components:t,...a}=e;return(0,r.mdx)("wrapper",(0,n.Z)({},p,a,{components:t,mdxType:"MDXLayout"}),(0,r.mdx)("h1",{id:"attrs-type"},(0,r.mdx)("inlineCode",{parentName:"h1"},"attrs")," type"),(0,r.mdx)("p",null,"This type is available as a global ",(0,r.mdx)("inlineCode",{parentName:"p"},"attrs")," symbol, to allow the definition of attributes to the ",(0,r.mdx)("inlineCode",{parentName:"p"},"rule")," function."),(0,r.mdx)("p",null,"As an example:"),(0,r.mdx)("pre",null,(0,r.mdx)("code",{parentName:"pre",className:"language-python"},'rule(impl = _impl, attrs = {"foo": attrs.string(), "bar": attrs.int(default = 42)})\n')),(0,r.mdx)("p",null,"Most attributes take at least two optional parameters:"),(0,r.mdx)("ul",null,(0,r.mdx)("li",{parentName:"ul"},(0,r.mdx)("p",{parentName:"li"},"A ",(0,r.mdx)("inlineCode",{parentName:"p"},"doc")," parameter, which specifies documentation for the attribute.")),(0,r.mdx)("li",{parentName:"ul"},(0,r.mdx)("p",{parentName:"li"},"A ",(0,r.mdx)("inlineCode",{parentName:"p"},"default")," parameter, which if present specifies the default value for the attribute if omitted.\nIf there is no default, the user of the rule must supply that parameter."))),(0,r.mdx)("p",null,"Each attribute defines what values it accepts from the user, and which values it gives to the rule.\nFor simple types like ",(0,r.mdx)("inlineCode",{parentName:"p"},"attrs.string")," these are the same, for more complex types like ",(0,r.mdx)("inlineCode",{parentName:"p"},"attrs.dep")," these\nare different (string from the user, dependency to the rule)."),(0,r.mdx)("h2",{id:"attrsany"},"attrs.any"),(0,r.mdx)("pre",null,(0,r.mdx)("code",{parentName:"pre",className:"language-python"},'def attrs.any(*, doc: str.type = "", default = _) -> "attribute"\n')),(0,r.mdx)("p",null,"Takes most builtin literals and passes them to the rule as a string. Discouraged, as it provides little type safety and destroys the structure."),(0,r.mdx)("hr",null),(0,r.mdx)("h2",{id:"attrsarg"},"attrs.arg"),(0,r.mdx)("pre",null,(0,r.mdx)("code",{parentName:"pre",className:"language-python"},'def attrs.arg(\n    *,\n    json: bool.type = False,\n    default = _,\n    doc: str.type = "",\n    anon_target_compatible: bool.type = False\n) -> "attribute"\n')),(0,r.mdx)("p",null,"Takes a command line argument from the user and supplies a ",(0,r.mdx)("inlineCode",{parentName:"p"},"cmd_args")," compatible value to the rule. The argument may contain special macros such as ",(0,r.mdx)("inlineCode",{parentName:"p"},"$(location :my_target)")," or ",(0,r.mdx)("inlineCode",{parentName:"p"},"$(exe :my_target)")," which will be replaced with references to those values in the rule. Takes in an optional ",(0,r.mdx)("inlineCode",{parentName:"p"},"anon_target_compatible")," flag, which indicates whether the args can be passed into anon targets. Note that there is a slight memory hit when using this flag."),(0,r.mdx)("hr",null),(0,r.mdx)("h2",{id:"attrsbool"},"attrs.bool"),(0,r.mdx)("pre",null,(0,r.mdx)("code",{parentName:"pre",className:"language-python"},'def attrs.bool(*, default = _, doc: str.type = "") -> "attribute"\n')),(0,r.mdx)("p",null,"Takes a boolean and passes it to the rule as a boolean."),(0,r.mdx)("hr",null),(0,r.mdx)("h2",{id:"attrsconfiguration_label"},"attrs.configuration","_","label"),(0,r.mdx)("pre",null,(0,r.mdx)("code",{parentName:"pre",className:"language-python"},'def attrs.configuration_label(*, doc: str.type = "") -> "attribute"\n')),(0,r.mdx)("hr",null),(0,r.mdx)("h2",{id:"attrsconfigured_dep"},"attrs.configured","_","dep"),(0,r.mdx)("pre",null,(0,r.mdx)("code",{parentName:"pre",className:"language-python"},'def attrs.configured_dep(\n    *,\n    providers: [""] = [],\n    default = _,\n    doc: str.type = ""\n) -> "attribute"\n')),(0,r.mdx)("hr",null),(0,r.mdx)("h2",{id:"attrsdefault_only"},"attrs.default","_","only"),(0,r.mdx)("pre",null,(0,r.mdx)("code",{parentName:"pre",className:"language-python"},'def attrs.default_only(\n    inner: "attribute",\n    /,\n    *,\n    doc: str.type = ""\n) -> "attribute"\n')),(0,r.mdx)("p",null,"Rejects all values and uses the default for the inner argument. Often used to resolve dependencies, which otherwise can't be resolved inside a rule."),(0,r.mdx)("pre",null,(0,r.mdx)("code",{parentName:"pre",className:"language-python"},'attrs.default_only(attrs.dep(default = "foo//my_package:my_target"))\n')),(0,r.mdx)("hr",null),(0,r.mdx)("h2",{id:"attrsdep"},"attrs.dep"),(0,r.mdx)("pre",null,(0,r.mdx)("code",{parentName:"pre",className:"language-python"},'def attrs.dep(\n    *,\n    providers: [""] = [],\n    default = _,\n    doc: str.type = ""\n) -> "attribute"\n')),(0,r.mdx)("p",null,"Takes a target from the user, as a string, and supplies a dependency to the rule. A target can be specified as an absolute dependency ",(0,r.mdx)("inlineCode",{parentName:"p"},"foo//bar:baz"),", omitting the cell (",(0,r.mdx)("inlineCode",{parentName:"p"},"//bar:baz"),") or omitting the package name (",(0,r.mdx)("inlineCode",{parentName:"p"},":baz"),")."),(0,r.mdx)("p",null,"If supplied the ",(0,r.mdx)("inlineCode",{parentName:"p"},"providers")," argument ensures that specific providers will be present\non the dependency."),(0,r.mdx)("hr",null),(0,r.mdx)("h2",{id:"attrsdict"},"attrs.dict"),(0,r.mdx)("pre",null,(0,r.mdx)("code",{parentName:"pre",className:"language-python"},'def attrs.dict(\n    key: "attribute",\n    value: "attribute",\n    *,\n    sorted: bool.type = False,\n    default = _,\n    doc: str.type = ""\n) -> "attribute"\n')),(0,r.mdx)("p",null,"Takes a dict from the user, supplies a dict to the rule."),(0,r.mdx)("hr",null),(0,r.mdx)("h2",{id:"attrsenum"},"attrs.enum"),(0,r.mdx)("pre",null,(0,r.mdx)("code",{parentName:"pre",className:"language-python"},'def attrs.enum(\n    variants: [str.type],\n    /,\n    *,\n    default = _,\n    doc: str.type = ""\n) -> "attribute"\n')),(0,r.mdx)("p",null,"Takes a string from one of the variants given, and gives that string to the rule. Strings are matched case-insensitively, and always passed to the rule lowercase."),(0,r.mdx)("hr",null),(0,r.mdx)("h2",{id:"attrsexec_dep"},"attrs.exec","_","dep"),(0,r.mdx)("pre",null,(0,r.mdx)("code",{parentName:"pre",className:"language-python"},'def attrs.exec_dep(\n    *,\n    providers: [""] = [],\n    default = _,\n    doc: str.type = ""\n) -> "attribute"\n')),(0,r.mdx)("p",null,"Takes a target from the user, as a string, and supplies a dependency to the rule. The dependency will transition to the execution platform. Use ",(0,r.mdx)("inlineCode",{parentName:"p"},"exec_dep")," if you plan to execute things from this dependency as part of the compilation."),(0,r.mdx)("hr",null),(0,r.mdx)("h2",{id:"attrsint"},"attrs.int"),(0,r.mdx)("pre",null,(0,r.mdx)("code",{parentName:"pre",className:"language-python"},'def attrs.int(*, default = _, doc: str.type = "") -> "attribute"\n')),(0,r.mdx)("p",null,"Takes an int from the user, supplies an int to the rule."),(0,r.mdx)("hr",null),(0,r.mdx)("h2",{id:"attrslabel"},"attrs.label"),(0,r.mdx)("pre",null,(0,r.mdx)("code",{parentName:"pre",className:"language-python"},'def attrs.label(*, default = _, doc: str.type = "") -> "attribute"\n')),(0,r.mdx)("p",null,"Takes a target (as per ",(0,r.mdx)("inlineCode",{parentName:"p"},"deps"),") and passes a ",(0,r.mdx)("inlineCode",{parentName:"p"},"label")," to the rule. Validates that the target exists, but does not introduce a dependency on it."),(0,r.mdx)("hr",null),(0,r.mdx)("h2",{id:"attrslist"},"attrs.list"),(0,r.mdx)("pre",null,(0,r.mdx)("code",{parentName:"pre",className:"language-python"},'def attrs.list(\n    inner: "attribute",\n    /,\n    *,\n    default = _,\n    doc: str.type = ""\n) -> "attribute"\n')),(0,r.mdx)("p",null,"Takes a list from the user, supplies a list to the rule."),(0,r.mdx)("hr",null),(0,r.mdx)("h2",{id:"attrsnamed_set"},"attrs.named","_","set"),(0,r.mdx)("pre",null,(0,r.mdx)("code",{parentName:"pre",className:"language-python"},'def attrs.named_set(\n    value_type: "attribute",\n    /,\n    *,\n    sorted: bool.type = False,\n    default = _,\n    doc: str.type = ""\n) -> "attribute"\n')),(0,r.mdx)("hr",null),(0,r.mdx)("h2",{id:"attrsone_of"},"attrs.one","_","of"),(0,r.mdx)("pre",null,(0,r.mdx)("code",{parentName:"pre",className:"language-python"},'def attrs.one_of(*args, default = _, doc: str.type = "") -> "attribute"\n')),(0,r.mdx)("p",null,"Given a list of alternative attributes, selects the first that matches and gives that to the rule."),(0,r.mdx)("hr",null),(0,r.mdx)("h2",{id:"attrsoption"},"attrs.option"),(0,r.mdx)("pre",null,(0,r.mdx)("code",{parentName:"pre",className:"language-python"},'def attrs.option(\n    inner: "attribute",\n    /,\n    *,\n    default = _,\n    doc: str.type = ""\n) -> "attribute"\n')),(0,r.mdx)("p",null,"Takes a value that may be ",(0,r.mdx)("inlineCode",{parentName:"p"},"None")," or some inner type, and passes either ",(0,r.mdx)("inlineCode",{parentName:"p"},"None")," or the value corresponding to the inner to the rule. Often used to make a rule optional:"),(0,r.mdx)("pre",null,(0,r.mdx)("code",{parentName:"pre",className:"language-python"},"attrs.option(attr.string(), default = None)\n")),(0,r.mdx)("hr",null),(0,r.mdx)("h2",{id:"attrsquery"},"attrs.query"),(0,r.mdx)("pre",null,(0,r.mdx)("code",{parentName:"pre",className:"language-python"},'def attrs.query(*, doc: str.type = "") -> "attribute"\n')),(0,r.mdx)("hr",null),(0,r.mdx)("h2",{id:"attrsregex"},"attrs.regex"),(0,r.mdx)("pre",null,(0,r.mdx)("code",{parentName:"pre",className:"language-python"},'def attrs.regex(*, default = _, doc: str.type = "") -> "attribute"\n')),(0,r.mdx)("p",null,"Currently an alias for ",(0,r.mdx)("inlineCode",{parentName:"p"},"attrs.string"),"."),(0,r.mdx)("hr",null),(0,r.mdx)("h2",{id:"attrsset"},"attrs.set"),(0,r.mdx)("pre",null,(0,r.mdx)("code",{parentName:"pre",className:"language-python"},'def attrs.set(\n    value_type: "attribute",\n    /,\n    *,\n    sorted: bool.type = False,\n    default = _,\n    doc: str.type = ""\n) -> "attribute"\n')),(0,r.mdx)("hr",null),(0,r.mdx)("h2",{id:"attrssource"},"attrs.source"),(0,r.mdx)("pre",null,(0,r.mdx)("code",{parentName:"pre",className:"language-python"},'def attrs.source(\n    *,\n    allow_directory: bool.type = False,\n    default = _,\n    doc: str.type = ""\n) -> "attribute"\n')),(0,r.mdx)("p",null,"Takes a source file from the user, supplies an artifact to the rule. The source file may be specified as a literal string (representing the path within this package), or a target (which must have a ",(0,r.mdx)("inlineCode",{parentName:"p"},"DefaultInfo")," with a ",(0,r.mdx)("inlineCode",{parentName:"p"},"default_outputs")," value)."),(0,r.mdx)("hr",null),(0,r.mdx)("h2",{id:"attrssplit_transition_dep"},"attrs.split","_","transition","_","dep"),(0,r.mdx)("pre",null,(0,r.mdx)("code",{parentName:"pre",className:"language-python"},'def attrs.split_transition_dep(\n    *,\n    providers: [""] = [],\n    cfg,\n    default = _,\n    doc: str.type = ""\n) -> "attribute"\n')),(0,r.mdx)("hr",null),(0,r.mdx)("h2",{id:"attrsstring"},"attrs.string"),(0,r.mdx)("pre",null,(0,r.mdx)("code",{parentName:"pre",className:"language-python"},'def attrs.string(\n    *,\n    default = _,\n    validate = _,\n    doc: str.type = ""\n) -> "attribute"\n')),(0,r.mdx)("p",null,"Takes a string from the user, supplies a string to the rule."),(0,r.mdx)("hr",null),(0,r.mdx)("h2",{id:"attrstoolchain_dep"},"attrs.toolchain","_","dep"),(0,r.mdx)("pre",null,(0,r.mdx)("code",{parentName:"pre",className:"language-python"},'def attrs.toolchain_dep(\n    *,\n    providers: [""] = [],\n    default = _,\n    doc: str.type = ""\n) -> "attribute"\n')),(0,r.mdx)("p",null,"Takes a target from the user, as a string, and supplies a dependency to the rule. The dependency will be a toolchain dependency, meaning that its execution platform dependencies will be used to select the execution platform for this rule."),(0,r.mdx)("hr",null),(0,r.mdx)("h2",{id:"attrstransition_dep"},"attrs.transition","_","dep"),(0,r.mdx)("pre",null,(0,r.mdx)("code",{parentName:"pre",className:"language-python"},'def attrs.transition_dep(\n    *,\n    providers: [""] = [],\n    cfg,\n    default = _,\n    doc: str.type = ""\n) -> "attribute"\n')),(0,r.mdx)("hr",null),(0,r.mdx)("h2",{id:"attrstuple"},"attrs.tuple"),(0,r.mdx)("pre",null,(0,r.mdx)("code",{parentName:"pre",className:"language-python"},'def attrs.tuple(*args, default = _, doc: str.type = "") -> "attribute"\n')),(0,r.mdx)("p",null,"Takes a tuple of values and gives a tuple to the rule."),(0,r.mdx)("hr",null),(0,r.mdx)("h2",{id:"attrsversioned"},"attrs.versioned"),(0,r.mdx)("pre",null,(0,r.mdx)("code",{parentName:"pre",className:"language-python"},'def attrs.versioned(\n    value_type: "attribute",\n    *,\n    doc: str.type = ""\n) -> "attribute"\n')))}u.isMDXComponent=!0}}]);