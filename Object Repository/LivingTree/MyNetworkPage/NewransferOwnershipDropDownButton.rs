<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>NewransferOwnershipDropDownButton</name>
   <tag></tag>
   <elementGuidId>bfb89e3a-d98c-45ad-8555-69dd01521dc6</elementGuidId>
   <selectorCollection>
      <entry>
         <key>XPATH</key>
         <value>//div[@id='page-content-wrapper']/div/div[4]/div[17]/div/form/fieldset/div/div/div[2]/div</value>
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
      <value>hoverMenu</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>

					

 					
                    Transfer Ownership
					

                    
                        function openReport(url) {
                          var win = window.open(url, &quot;_blank&quot;);
                          win.focus();
                        }
                    

                    
                    Download Active Users
                    
                    Download Inactive Users
                                    
                                    function downloadGroupPhotos() {
                                            if(confirm(&quot;Photos for this group will be bundled into a single compressed file and a link to the file will be sent via email.  This process may take several minutes depending on how many photos are in the group.  Do you want to continue?&quot;)){
                                                    $.post(&quot;https://maple.livingtree.com/action/groups/download_group_photos&quot;, {download_group_guid:28549445}, function(data, textStatus) {

                                                    if(data.status == &quot;success&quot;){
                                                            alert(&quot;Your request has been submitted.  You will receive an email with the link to your files shortly.&quot;);
                                                    } else {
                                                            alert(&quot;An error occurred processing the request. Please try again.&quot;);
                                                    }

                                                    }, &quot;json&quot;);
                                            }
                                    }
                                    

                                    
                                    Download Photos
                    


 					

						$(document).ready(function(){

							$(&quot;:not(.hoverMenu, #hoverMenuItems, #hoverMenuItems div, #hoverMenuItems hr)&quot;).hover(function(){
								$(&quot;#hoverMenuItems&quot;).hide(30);
							});

							$(&quot;.hoverMenu, #hoverMenuItems&quot;).click(function(){

								$(&quot;#hoverMenuItems&quot;).show(10, function(){
									var hoverMenuItems = $( &quot;.hoverMenu&quot; );
									var hoverMenuPosition = hoverMenuItems.position();
									$(&quot;#hoverMenuItems&quot;).css({left: hoverMenuPosition.left - 150});
								});

							});


							$(&quot;#hoverButton&quot;).hover(function(){
								$(this).css(&quot;color&quot;, &quot;#adadad&quot;);
							});

							$(&quot;#hoverButton&quot;).mouseout(function(){
								$(this).css(&quot;color&quot;, &quot;#999&quot;);
							});

						});

                    


					</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;page-content-wrapper&quot;)/div[@class=&quot;elgg-main elgg-body post-container&quot;]/div[@class=&quot;build-allmembergroups&quot;]/div[@class=&quot;elgg-module  elgg-module-member-types&quot;]/div[@class=&quot;elgg-body&quot;]/form[@class=&quot;elgg-form&quot;]/fieldset[1]/div[@class=&quot;member-group-block&quot;]/div[@class=&quot;membertype-header&quot;]/div[@class=&quot;membertype-header-right-block&quot;]/div[@class=&quot;hoverMenu&quot;]</value>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:idRelative</name>
      <type>Main</type>
      <value>//div[@id='page-content-wrapper']/div/div[4]/div[17]/div/form/fieldset/div/div/div[2]/div</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='All Administrators'])[1]/following::div[2]</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Administrators'])[1]/following::div[2]</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//fieldset/div/div/div[2]/div</value>
   </webElementXpaths>
</WebElementEntity>
