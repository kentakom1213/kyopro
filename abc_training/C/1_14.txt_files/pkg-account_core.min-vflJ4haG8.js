define("modules/clean/account/email_verify_reasons",["require","exports"],(function(e,t){"use strict";Object.defineProperty(t,"__esModule",{value:!0}),t.EmailVerificationReasons=void 0,(function(e){e.SHARE_FOLDER="share_folder",e.CREATE_API_APP="create_api_app",e.PUBLIC_FOLDER="public_folder",e.GENERIC="generic",e.SHMODAL="shmodal",e.SHARE_FILEVIEWER="share_fileviewer",e.MOBILE_SHARE_FOLDER="mobile_share_folder",e.EMAIL_ALIAS="email_alias",e.CHANGE_EMAIL="change_email",e.PROMPT_CAMPAIGN="prompt_campaign",e.ADD_COMMENT="add_comment",e.SUBSCRIBE_TO_COMMENTS="subscribe_to_comments",e.CREATE_FILE_COLLECTOR="create_file_collector",e.JOIN_DISCOVERED_TEAM="join_discovery",e.CREATE_TEAM="create_team",e.NEW_DFB_TEAM_TRY="new_dfb_team_try",e.NEW_DFB_TEAM_BUY="new_dfb_team_buy",e.GIFT_BUY="gift_buy",e.REFER_FRIENDS="refer_friends",e.UJ_VERIFY_EMAIL="uj_verify_email",e.SHOWCASE_USER="showcase_user",e.CLOUD_DOCS="cloud_docs",e.REVERIFICATION_CAMPAIGN="reverification_campaign",e.CHANGE_EMAIL_FOR_APPLE_SIGNUP="change_email_for_apple_signup",e.SUSPICIOUS_ALPACA_REVERIFICATION="suspicious_alpaca_reverification",e.SEND_TRANSFER_VIA_EMAIL="send_transfer_via_email",e.SIGN_UP_FOR_SPACES="sign_up_for_spaces",e.OAUTH_CONFIRM="oauth_confirm",e.POST_PURCHASE="post_purchase",e.ONBOARDING_SURVEY="onboarding_survey",e.MOUNT_FOLDER="mount_folder"})(t.EmailVerificationReasons||(t.EmailVerificationReasons={}))})),define("modules/clean/account_page/components/account_table",["require","exports","tslib","react","classnames","styled-components"],(function(e,t,a,o,n,l){"use strict";Object.defineProperty(t,"__esModule",{value:!0}),t.Table=t.TableFooter=t.TableBody=t.TableHead=t.HeaderRow=t.BodyRow=t.HeaderCell=t.BodyCell=void 0,o=a.__importDefault(o),n=a.__importDefault(n);const i=(l=a.__importDefault(l)).default.div`
  border: 1px solid var(--color__faint__border);
  border-radius: 0;
`,r=l.default.tr`
  width: 100%;
  border-collapse: collapse;
  color: var(--color__faint__text);
  height: var(--spacing__unit--5);
  background-color: var(--color__faint__background);
  font-weight: normal;
`,s=l.default.tr`
  height: 56px;
  border-top: 1px solid var(--color__faint__border);
  width: 100%;
  border-collapse: collapse;
`,c=l.default.td`
  padding: var(--spacing__unit--2);
  color: var(--color__black);
  font-size: var(--type__body__standard--fontsize)
  text-align: left;
  width: 100%;
  border-collapse: collapse;
  &:last-child {
    padding-right: var(--spacing__unit--2);
  }
`,_=l.default.th`
  width: 100%;
  border-collapse: collapse;
  padding: var(--spacing__unit--2);
  font-size: var(--type__body__standard--fontsize) !important;
  background-color: var(--color__faint__background);
  font-weight: normal;
  text-align: left;
  color: var(--color__faint__text);
  &:last-child {
    padding-right: var(--spacing__unit--2);
  }
`;t.BodyCell=e=>{var{className:t}=e,l=a.__rest(e,["className"]);const i=n.default("account-maestro-table__body-cell",t);return o.default.createElement(c,Object.assign({className:i},l))},t.HeaderCell=e=>{var{className:t}=e,l=a.__rest(e,["className"]);const i=n.default("account-maestro-table__header-cell",t);return o.default.createElement(_,Object.assign({className:i},l))},t.BodyRow=e=>{var{className:t}=e,l=a.__rest(e,["className"]);const i=n.default("account-maestro-table__body-row",t);return o.default.createElement(s,Object.assign({className:i},l))},t.HeaderRow=e=>{var{className:t}=e,l=a.__rest(e,["className"]);const i=n.default("account-maestro-table__header-row",t);return o.default.createElement(r,Object.assign({className:i},l))},t.TableHead=e=>o.default.createElement("thead",Object.assign({},e)),t.TableBody=e=>o.default.createElement("tbody",Object.assign({},e)),t.TableFooter=e=>o.default.createElement("tfoot",Object.assign({},e)),t.Table=e=>{var{className:t}=e,l=a.__rest(e,["className"]);const r=n.default("account-maestro-table",t);return o.default.createElement(i,{className:"account-maestro-table-container"},o.default.createElement("table",Object.assign({className:r},l)))}})),define("modules/clean/account_page/components/action_table",["require","exports","tslib","react","classnames","styled-components","dig-components/buttons","dig-components/icons","dig-components/icons/src","modules/clean/account_page/components/account_table"],(function(e,t,a,o,n,l,i,r,s,c){"use strict";Object.defineProperty(t,"__esModule",{value:!0}),t.ActionTable=void 0,o=a.__importDefault(o),n=a.__importDefault(n);const _=(l=a.__importDefault(l)).default(c.BodyCell)`
  .mc-tertiary-icon-text {
    line-height: var(--type__body__standard--lineheight);
    height: var(--spacing__unit--3);
  }
`;class u extends o.default.Component{constructor(){super(...arguments),this.onClose=e=>()=>{this.props.onClosePressed(e)}}renderBodyRows(){var e;const t=null===(e=this.props.hasActionsColumn)||void 0===e||e;return(this.props.sortFunction?this.props.data.sort(this.props.sortFunction):this.props.data).map((e,a)=>{const n=null!=this.props.hideClose&&this.props.hideClose(e);return o.default.createElement(c.BodyRow,{key:a},this.props.keys.map((t,n)=>o.default.createElement(c.BodyCell,{key:`${a}-${n}`},this.props.renderRowDetail(e,t))),t&&o.default.createElement(_,{key:`${a}--1`},!n&&o.default.createElement(i.IconButton,{variant:"transparent","aria-label":this.props.dismissAriaLabel(e),onClick:this.onClose(e),className:"action-table__button"},o.default.createElement(r.UIIcon,{src:s.DeleteLine}))))})}renderHeaderCells(){return this.props.keys.map((e,t)=>o.default.createElement(c.HeaderCell,{key:t},this.props.headers[e]))}render(){var e;const t=n.default("action-table",this.props.tableClass),a=null===(e=this.props.hasActionsColumn)||void 0===e||e;return o.default.createElement(c.Table,{className:t},o.default.createElement(c.TableHead,null,o.default.createElement(c.HeaderRow,null,this.renderHeaderCells(),a&&o.default.createElement(c.HeaderCell,{"aria-label":this.props.dismissHeaderAriaLabel}))),o.default.createElement(c.TableBody,null,this.renderBodyRows()))}}t.ActionTable=u,u.displayName="ActionTable"})),define("modules/clean/account_page/components/button_block",["require","exports","tslib","react","modules/clean/account_page/components/info_tooltip","modules/clean/react/components/css","dig-components/buttons","modules/core/browser","dig-components/typography","styled-components"],(function(e,t,a,o,n,l,i,r,s,c){"use strict";Object.defineProperty(t,"__esModule",{value:!0}),t.ButtonBlock=void 0,o=a.__importDefault(o),r=a.__importStar(r);const _=(c=a.__importDefault(c)).default(s.Text)`
  display: flex;
  align-items: center;
`,u=c.default.div`
  height: 96px;
  padding-left: var(--spacing__base_unit);
  padding-right: var(--spacing__base_unit);

  a.mc-button-secondary {
    text-decoration: none;
  }
`,d=c.default.span`
  &&& {
    padding-left: 0;
    padding-right: 0;
  }
`,p=c.default.div`
  display: flex;
  flex-direction: column;
  justify-content: center;
  flex: 1;
`,m=c.default.div`
  &&& {
    margin: var(--spacing__unit--0_5) 0 0 0;
    max-width: 400px;
    padding: 0;
  }
`,f=c.default(i.Button)`
  .mc-tertiary-icon-text {
    line-height: var(--type__body__standard--lineheight);
  }
`;t.ButtonBlock=l.requireCssWithComponent(e=>{const{kind:t,buttonAction:a,disabled:l,buttonText:i,className:s,subtext:c,hideClickOption:g,italicizedSubtext:h,label:b,tooltipText:v,ariaButtonLabel:y}=e,x="link"===a.kind?a.href:void 0,E={className:"account-button-block__button",onClick:function(e){x&&r.open_tab(x),"function"===a.kind&&a.handler.call(e.target,e)},disabled:l},k="outline"===t,T=e["data-test"];let N="transparent";k&&(N="outline");const O=["account-page-module ","account-button-block",s].join(" ");return o.default.createElement(u,{className:O},o.default.createElement(p,{className:"account-button-block__label"},o.default.createElement(d,{className:"account-page-block__heading"},o.default.createElement(m,{className:"account-page-block__subtext"},o.default.createElement(_,{size:"standard"},o.default.createElement("div",null,b),v&&o.default.createElement(n.InfoTooltip,{content:v,position:n.TooltipPlacement.BOTTOM})))),c&&o.default.createElement(m,{className:"account-page-block__subtext"},c,h&&o.default.createElement(o.default.Fragment,null,o.default.createElement("br",null),o.default.createElement("i",null,h)))),!g&&o.default.createElement(f,Object.assign({},E,{variant:N,"data-test":T,"aria-label":y}),i))},["/static/css/spectrum/index.web-vfliw9181.css"])})),define("modules/clean/account_page/components/info_tooltip",["require","exports","tslib","react","modules/core/i18n","dig-components/tooltips","dig-components/buttons","dig-components/icons/src","dig-components/icons","dig-components/typography","styled-components"],(function(e,t,a,o,n,l,i,r,s,c,_){"use strict";Object.defineProperty(t,"__esModule",{value:!0}),t.InfoTooltip=t.TooltipPlacement=void 0,o=a.__importDefault(o);const u=(_=a.__importDefault(_)).default(c.Text)`
  &&& {
    display: flex;
    align-items: center;
  }
`,d=_.default.span`
  display: flex;
  align-items: center;
`,p=_.default.span`
  margin-right: var(--spacing__base_unit);
`;t.TooltipPlacement={TOP:"top",RIGHT:"right",BOTTOM:"bottom",LEFT:"left"},t.InfoTooltip=function(e){const a=["info-tooltip",e.className].join(" "),c=e.position||t.TooltipPlacement.RIGHT,[_]=o.default.useState(c);return o.default.createElement(d,{className:a},e.prompt&&o.default.createElement(p,null,e.prompt),o.default.createElement(l.Tooltip,{placement:_,title:e.content},o.default.createElement(i.IconButton,{variant:"transparent"},o.default.createElement(u,{color:"faint"},o.default.createElement(s.UIIcon,{"aria-label":n.intl.formatMessage({id:"+SX5vq",defaultMessage:"More information"}),className:"info-tooltip__icon",tabIndex:0,src:r.InfoLine})))))}})),define("modules/clean/account_page/components/key_value_block",["require","exports","tslib","react","dig-components/buttons","dig-components/typography","modules/clean/account_page/components/info_tooltip","styled-components"],(function(e,t,a,o,n,l,i,r){"use strict";Object.defineProperty(t,"__esModule",{value:!0}),t.KeyValueBlock=void 0,o=a.__importDefault(o);const s=(r=a.__importDefault(r)).default(l.Text)`
  display: flex;
  align-items: center;
`,c=r.default.div`
  min-height: 72px;
  white-space: pre;
  padding-left: var(--spacing__base_unit);
  padding-right: var(--spacing__base_unit);

  &.inline_mode {
    border: 0;
    padding: 0 0 2px var(--spacing__base_unit);
    height: auto;
    min-height: var(--spacing__unit--4);
  }
`,_=r.default.span`
  color: var(--color__faint__text);
`,u=r.default.span`
  flex-grow: 1;
  white-space: normal;
`,d=r.default(l.Text)`
  font-size: var(--type__body__small--fontsize);
  color: var(--color__faint__text);
  margin: var(--spacing__unit--0_5) 0 var(--spacing__unit--0_5) 0;
`,p=r.default.div`
  display: flex;

  > :not(:last-child) {
    margin-right: var(--spacing__unit--2);
  }
`,m=r.default(l.Text)`
  margin-right: var(--spacing__unit--2);
`;t.KeyValueBlock=e=>{let t;t=!e.valueText&&e.placeholder?o.default.createElement(_,{className:"account-key-value-block__placeholder"},e.placeholder):o.default.createElement("span",null,e.valueText);const a=e.inlineMode?"inline_mode":"",l=["account-page-module","account-key-value-block",e.className,a].join(" ");return o.default.createElement(c,{className:l},!e.inlineMode&&o.default.createElement(u,{className:"account-key-value-block__key"},o.default.createElement(s,{size:"standard",className:"account-key-value-block__heading"},e.keyText,e.tooltipText&&o.default.createElement(i.InfoTooltip,{content:e.tooltipText,position:i.TooltipPlacement.BOTTOM})),e.subtext&&o.default.createElement(d,{className:"account-key-value-block__subtext",size:"small",color:"faint"},e.subtext||"")),o.default.createElement(m,{tagName:"div",size:"standard",className:"account-key-value-block__value"},t),o.default.createElement(p,{className:"account-key-value-block__links"},e.linkText&&o.default.createElement(n.Button,{className:"account-key-value-block__link",onClick:e.onLinkClick,disabled:e.disabled,variant:e.linkTextVariant||"transparent","aria-label":e.ariaPrimaryButtonLabel},e.linkText),e.secondLink&&o.default.createElement(n.Button,{className:"account-key-value-block__link",onClick:e.onSecondLinkClick,disabled:e.disabled,variant:"transparent","aria-label":e.ariaSecondaryButtonLabel},e.secondValueText),e.customControl))}})),define("modules/clean/account_page/components/loading",["require","exports","tslib","react","modules/core/i18n","dig-components/progress_indicators","styled-components"],(function(e,t,a,o,n,l,i){"use strict";Object.defineProperty(t,"__esModule",{value:!0}),t.Loading=void 0,o=a.__importDefault(o);const r=(i=a.__importDefault(i)).default.div`
  display: flex;
  justify-content: center;
  align-items: center;
`,s=i.default(l.Spinner)`
  width: 100%;
  margin-top: 100px;
  margin-bottom: 100px;
  display: flex;
  align-items: center;
  justify-content: center;
`;t.Loading=()=>o.default.createElement(r,null,o.default.createElement(s,{className:"account-loading-indicator","aria-valuetext":n.intl.formatMessage({id:"jybGUg",defaultMessage:"Loading"})}))})),define("modules/clean/account_page/components/toggle_block",["require","exports","tslib","react","modules/clean/account_page/components/key_value_block","dig-components/controls","dig-components/typography","modules/core/i18n","styled-components"],(function(e,t,a,o,n,l,i,r,s){"use strict";Object.defineProperty(t,"__esModule",{value:!0}),t.ToggleBlock=void 0,o=a.__importDefault(o);const c=(s=a.__importDefault(s)).default(i.Text)`
  margin-right: var(--spacing__base_unit);
`,_=s.default(n.KeyValueBlock)`
  height: 72px;
  padding-left: var(--spacing__base_unit);
  padding-right: var(--spacing__base_unit);

  .account-toggle-block__label {
    flex-grow: 1;
    display: flex;
    justify-content: center;
    flex-direction: column;
  }

  .account-toggle-block__tooltip-label {
    display: flex;
    flex-direction: row;
    align-items: center;

    .info-tooltip {
      margin-left: var(--spacing__unit--0_5);
    }
  }

  .account-toggle-block__heading {
    font-size: var(--type__body__standard--fontsize);
    color: var(--color__standard__text);
  }

  .account-toggle-block__subtex {
    font-size: var(--type__body__small--fontsize);
    color: var(--color__faint__text);
    margin-top: var(--spacing__unit--0_5);
  }

  .account-toggle-block__detail {
    margin-right: var(--spacing__unit--2);
  }

  .toggle:not(.toggle-block__toggle) {
    height: var(--spacing__unit--4);
  }

  .toggle {
    display: flex;
    align-items: center;
    cursor: pointer;
    line-height: 1.5em;

    &:focus {
      box-shadow: 0 0 0 3px var(--color__theme__primary--lighter);
    }
  }

  .toggle__status {
    display: inline-block;
    height: 20px;
    margin-right: var(--spacing__base_unit);
    font-weight: var(--type__body__standard--fontweight);
    color: var(--color__faint__text);
  }

  .toggle__status--on {
    font-weight: var(--type__body__standard--fontweight_strong);
    color: var(--color__dbx__blue--100);
  }

  .toggle__toggle-container {
    box-sizing: border-box;
    transition: all 0.15s ease-in-out;
    display: inline-block;
    position: relative;
    height: 20px;
    width: 36px;
    border-radius: var(--spacing__unit--0_5);
    background-color: var(--color__faint__background);
  }

  .toggle__status--on {
    background-color: var(--color__dbx__blue--100);
  }

  .toggle__toggle {
    transition: transform 0.15s ease-in-out;
    position: absolute;
    top: var(--spacing__unit--0_5);
    left: var(--spacing__unit--0_5);
    height: var(--spacing__unit--1_5);
    width: var(--spacing__unit--1_5);
    background-color: var(--color__standard__background);
    border-radius: 2px;
    transform: translateX(0);
  }

  .toggle__status--on {
    transform: translateX(var(--spacing__unit--2));
  }
`;t.ToggleBlock=e=>{const t=["account-page-module","account-toggle-block",e.className].join(" ");let a=e.customStatusDescription;return a||(a={onText:r.intl.formatMessage({id:"qboPoB",defaultMessage:"On"}),offText:r.intl.formatMessage({id:"ZSM6XN",defaultMessage:"Off"})}),o.default.createElement(_,{className:t,keyText:e.label,subtext:e.subLabel,valueText:e.detail,tooltipText:e.tooltipText,customControl:o.default.createElement(o.default.Fragment,null,o.default.createElement(c,{size:"standard"},e.isOn?null==a?void 0:a.onText:null==a?void 0:a.offText),o.default.createElement(l.Toggle,{className:["toggle toggle-block__toggle",e.toggleClass].join(" "),isToggled:e.isOn,onChange:t=>{const a=t.target.checked;e.onToggle(a)},disabled:e.isDisabled,wrapperProps:{"aria-pressed":e.isOn}}))})}})),define("modules/clean/account_page/util",["require","exports","tslib","modules/core/browser","modules/clean/viewer","modules/clean/active_user","modules/clean/analytics"],(function(e,t,a,o,n,l,i){"use strict";Object.defineProperty(t,"__esModule",{value:!0}),t.logClickAndRedirect=t.logClickAndOpenTab=t.getTeamInfo=void 0,o=a.__importStar(o),t.getTeamInfo=function(){const e=n.Viewer.get_viewer(),{is_team:t}=l.mustGetActiveUser(),{team_dbtid:a,team_name:o}=e;return t&&void 0!==a&&void 0!==o?{teamId:a,teamName:o}:void 0},t.logClickAndOpenTab=(e,t,a)=>()=>{i.ProEventsLogger.log(e,a,()=>{o.open_tab(t)})},t.logClickAndRedirect=(e,t,a)=>()=>{i.ProEventsLogger.log(e,a,()=>{o.redirect(t)})}})),define("modules/clean/deprecated_pyxl/controllers/phone_number",["require","exports","tslib","jquery","modules/clean/web_timing_logger","modules/clean/deprecated_pyxl/controller_helpers","modules/core/i18n"],(function(e,t,a,o,n,l,i){"use strict";Object.defineProperty(t,"__esModule",{value:!0}),o=a.__importDefault(o),n=a.__importStar(n),l=a.__importStar(l);t.default=class{constructor(t,o){this.set=this.set.bind(this),this._setup_valhooks=this._setup_valhooks.bind(this),this.$phone_input=t,this.$error_span=null,this.id=this.$phone_input.attr("id"),this.$country_select=this.$phone_input.find(".phone-country"),this.$phone_text=this.$phone_input.find(".phone-text"),this._setup_valhooks(),this._listen(),this._phone_numbers=void 0,o?n.waitForTTI().then(()=>new Promise((t,a)=>{e(["phone_helpers"],t,a)}).then(a.__importStar).then(({default:e})=>this._phone_numbers=e)):new Promise((t,a)=>{e(["phone_helpers"],t,a)}).then(a.__importStar).then(({default:e})=>this._phone_numbers=e)}_listen(){return this._reformat(),this.$phone_text.on("blur",()=>(this._reformat(),this.validate_on_blur())),this.$country_select.on("change",()=>(this._reformat(),this.validate_on_blur())),this.$phone_text.on("focus",()=>this.hide_error())}_get_example_number(){return this._phone_numbers?this._phone_numbers.get_example_mobile_number(this.get_country_code()):"(201) 555-0123"}_get_country_code(){return this._phone_numbers?this._phone_numbers.iso_to_dialing(this.$country_select.val()||"US"):"1"}_split_number(e){return this._phone_numbers?this._phone_numbers.split_number(e):e}_is_valid(e){return!this._phone_numbers||this._phone_numbers.is_valid(e)}_full_number(){const e=this.$country_select.val(),t=this.$phone_text.val();return this._phone_numbers?this._phone_numbers.full_number(e,t):t}_format(e,t){return this._phone_numbers?this._phone_numbers.format(e,t):`${e} ${t}`}_reformat(){this.hide_error();const e=this.$phone_text.val();if(!e){const e=this._get_example_number();return void o.default("label",this.$phone_text).text(i.intl.formatMessage({id:"xjc3q9",defaultMessage:"Example:"})+e)}const t=this._format(this.get_country_code(),e);if(t)return this.$phone_text.val(t)}get_country_code(){return this._get_country_code()}get_local_number(){const e=this.$phone_input.val();return this._split_number(e).phone_number}is_empty(){return!this.$phone_text.val()}show_error(e){const t=this.$phone_input.find(".phone-number-error");return t.css("display","block"),t.text(e)}hide_error(){return this.$phone_input.find(".phone-number-error").html("&nbsp;")}is_valid_french_polynesian_number(e){return"+689"===e.slice(0,4)&&(6===(e=(e=e.slice(4)).replace(/[^\d]/g,"")).length||8===e.length)}validate_on_blur(){if(this.is_empty())return this.hide_error(),!0;const e=this.$phone_input.val();return this.is_valid_french_polynesian_number(e)?(this.hide_error,!0):this._is_valid(e)?(this.hide_error(),!0):(this.show_error(i.intl.formatMessage({id:"Q8gucb",defaultMessage:"Invalid phone number"})),!1)}validate_on_submit(){return!!this.validate_on_blur()&&(!this.is_empty()||(this.show_error(i.intl.formatMessage({id:"8AK5Pp",defaultMessage:"Please enter a phone number"})),!1))}focus(){return this.$phone_text.focus()}get(){if(this.is_empty())return null;try{return this._full_number(this.$country_select.val(),this.$phone_text.val())}catch(e){return null}}set(e){if(!e)return this.$phone_text.val(""),void this._reformat();try{const t=this._split_number(e);return t.iso_code?(this.$country_select.val(t.iso_code),this.$phone_text.val(this._format(t.dialing_code,t.phone_number)),void this._reformat()):void this.$phone_text.val(e)}catch(t){return this.$phone_text.val(e),this.show_error(i.intl.formatMessage({id:"Q8gucb",defaultMessage:"Invalid phone number"}))}}_setup_valhooks(){return this.$phone_input.each((function(){return this.type="phoneinput"})),o.default.valHooks.phoneinput={get:e=>l.get_controller(o.default(e)).get(),set:(e,t)=>l.get_controller(o.default(e)).set(t)}}_validate_phone_number(e,t){return t=t.replace(/\D/g,""),this._format(e,t)}}}));
//# sourceMappingURL=pkg-account_core.min.js-vfl5iClFD.map