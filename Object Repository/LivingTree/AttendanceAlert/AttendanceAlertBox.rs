<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>AttendanceAlertBox</name>
   <tag></tag>
   <elementGuidId>a0813eaa-0d35-4b97-a8b7-3cd452dd162e</elementGuidId>
   <selectorCollection>
      <entry>
         <key>XPATH</key>
         <value>//div[@id='page-content-wrapper']/div</value>
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
		
	Attendance Alerts

    
            


	
		
                    Please upload a comma delimited text file(CSV) to send a batch of attendance alerts. Attendance alerts will be delivered to all parents/guardians connected to a student via a private LivingTree Inbox message and an automated phone call(where the parent/guardian allows phone calls from LivingTree). Alerts will be queued immediately upon approval. Click here to download a sample CSV file.			
		
	


	
                                
		
			
				Select a school:
				
					
                                        Select School
                                            
					hinduschool
                                        					
				
			
			
				
			
			
				Select a file to upload:
				
					
				
			
			
				
			
			
				
				
				    
				
			
			
				
			
		
                    
	


        Previous Alert Logs:


	
		                            
                                                        07/29/2020
                                                        
                        
                        	




$(document).ready(function(){
        $(&quot;.elgg-menu-item-3-groups-user-invites&quot;).html('&lt;a href=&quot;https://maple.livingtree.com/attendance/upload&quot; >Attendance Alerts&lt;/a>');
        $(&quot;.elgg-menu-item-3-groups-user-invites&quot;).addClass('elgg-state-selected');
});

function loading(){
$(&quot;body&quot;).append('&lt;div id=&quot;overlay&quot; style=&quot;position: absolute; left: 0; top: 0; bottom: 0; right: 0; background:#fff;padding:2px 1px;border-radius:50%;width:64px;height:64px; opacity: 0.8; filter: alpha(opacity=80);&quot;>	&lt;img id=&quot;loading&quot; src=&quot;&quot; style=&quot;width: 60px;border: 0px;box-shadow: none; position: absolute; top: 50%; left: 50%; margin: -28px 0 0 -25px;&quot;>&lt;/div>');
}





$(document).ready(function(){
	$(&quot;#attendance_upload&quot;).submit(function(e){
		
		var error = &quot;&quot;;
		
		if($(error == &quot;&quot; &amp;&amp; &quot;#list_schools&quot;).val() == 0){
			error = &quot;Please select a school.&quot;;
		}
		
		if(error == &quot;&quot; &amp;&amp; $(&quot;#attendance_file&quot;).val() == &quot;&quot;){
			error = &quot;Please select a file.&quot;;
		}
		
		if(error == &quot;&quot;){
			var ext = $('#attendance_file').val().split('.').pop().toLowerCase();
			if($.inArray(ext, ['csv']) == -1) {
			    	error = &quot;Please select a CSV file.&quot;;
			}
		}
		
		if(error != &quot;&quot;){
			e.preventDefault();
			$(&quot;#error_message&quot;).html('&lt;div style=&quot;padding:5px;width:100%;background-color: #FEF5AA;border: 1px sandybrown solid;color:red;font-weight: bold;&quot;>'+error+'&lt;/div>');
                        return FALSE;
		}
                
                loading();
	});
});




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
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Logout'])[2]/following::div[14]</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Help'])[1]/following::div[14]</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//div/div/div[2]/div</value>
   </webElementXpaths>
</WebElementEntity>
