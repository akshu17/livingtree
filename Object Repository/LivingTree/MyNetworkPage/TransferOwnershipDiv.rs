<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>TransferOwnershipDiv</name>
   <tag></tag>
   <elementGuidId>55c36200-2c9d-4a2c-8cda-71ef5ee714a3</elementGuidId>
   <selectorCollection>
      <entry>
         <key>XPATH</key>
         <value>//div[@id='hoverMenuItems']</value>
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
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>id</name>
      <type>Main</type>
      <value>hoverMenuItems</value>
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
                    </value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;hoverMenuItems&quot;)</value>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:attributes</name>
      <type>Main</type>
      <value>//div[@id='hoverMenuItems']</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:idRelative</name>
      <type>Main</type>
      <value>//div[@id='page-content-wrapper']/div/div[4]/div[17]/div/form/fieldset/div/div/div[2]/div/div</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='All Administrators'])[1]/following::div[3]</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Administrators'])[1]/following::div[3]</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//fieldset/div/div/div[2]/div/div</value>
   </webElementXpaths>
</WebElementEntity>
