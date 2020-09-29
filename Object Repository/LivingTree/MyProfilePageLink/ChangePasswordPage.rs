<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>ChangePasswordPage</name>
   <tag></tag>
   <elementGuidId>c2f7c180-d061-4a7e-958e-9eb8df15abec</elementGuidId>
   <selectorCollection>
      <entry>
         <key>XPATH</key>
         <value>`````````</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>div</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>elgg-main elgg-body post-container</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
		 
Change My Password


Current Password

New Password

Confirm Password



(function($) {
 $(&quot;#changepwd_form&quot;).attr(&quot;autocomplete&quot;,&quot;off&quot;);
 $.fn.placeholder = function() {
 if(typeof document.createElement(&quot;input&quot;).placeholder == 'undefined') {
 $('[placeholder]').focus(function() {
 var input = $(this);
 if (input.val() == input.attr('placeholder')) {
 input.val('');
 input.removeClass('placeholder');
 }
 }).blur(function() {
 var input = $(this);
 if (input.val() == '' || input.val() == input.attr('placeholder')) {
 input.addClass('placeholder');
 if(input.attr('type') != &quot;password&quot;)
	 	input.val(input.attr('placeholder'));
 }
 }).blur().parents('form').submit(function() {
 $(this).find('[placeholder]').each(function() {
 var input = $(this);
 if (input.val() == input.attr('placeholder')) {
 input.val('');
 }
 })
 });
 }
 }
 })(jQuery);
 
$.fn.placeholder(); 






	</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;page-content-wrapper&quot;)/div[@class=&quot;elgg-main elgg-body post-container&quot;]</value>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:idRelative</name>
      <type>Main</type>
      <value>//div[@id='page-content-wrapper']/div</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Overall Snapshot'])[1]/following::div[2]</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Manage Data'])[2]/following::div[2]</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//div/div/div[2]/div</value>
   </webElementXpaths>
</WebElementEntity>
