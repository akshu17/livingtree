<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>ManageDataPage</name>
   <tag></tag>
   <elementGuidId>714afe70-bbb6-4e27-9439-aba495ad4cf4</elementGuidId>
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
		Manage Data



	


	.elgg-main.elgg-body.post-container{
		overflow: hidden !important;
	}
	.self-service-field-section{
		display: none;
	}
	.alert{
		border: none !important;
		border-radius: 0px;
	}

	
			
							
				
										Select School...
					Pleasant Hill Elementary SchoolBlackshear ElementaryMR Elementary School				
			
		
		
	
		
				
				School Type
			
		
		
				
				
									
						
													
								
								Elementary School							
							
													
								
								Middle School							
							
													
								
								High School							
							
						
					
							
			
		
		
		
				
				Principal Details
			
		
		
				
				
					
						
							First Name
							Last Name
							Email
						
					
					
												
							
							
								
											
					
				
			
			
		
				
				Other School Admins (Optional)
			
		
		
				
				
					
						
							
							First Name
							Last Name
							Email
							
							
						
					
					
												
							
							
								
							
											
					
				
			
		
		
				
			
				Add Admin
			
			
		
		
				
				Data Files
				
			
		
		
				
				
					
						
							File
							Last Updated
							
							
						
					
					
						
							Students
							---
							 
								
								Upload
													
						
						
							Teachers
							---
							 
									
								Upload
							
							
						
						
							Classes
							---
							 
								
								Upload
							
							
						
						
							Class Rosters
							---
							 
								
								Upload
													
						
						
							Student Contacts
							---
							 
								
								Upload
													
						
						
							Grade/Layers (Optional)
							---
							 
								
								Upload
													
						
						
							Other Class Admins (Optional)
							---
							 
								
								Upload
													
						
					
				
			
		
		
			
				
			
		
		
				
					
					Update Network
				
					Save Without Updating Network
					
			
		


	



(function($) {
	$(document).ready(function () {
		//var counter = 1;
		//schooladmincounter =  1;
		if(typeof schooladmincounter === 'undefined')
		{
			counter = 2;
		}
		else
		{
			counter = schooladmincounter;
		}
		
		$(&quot;#addrow&quot;).on(&quot;click&quot;, function () {
			var newRow = $(&quot;&lt;tr>&quot;);
			var cols = &quot;&quot;;

			cols += '&lt;td>&lt;input type=&quot;text&quot; class=&quot;form-control schooladminfirstname&quot; id=&quot;schooladminfirstname-'+counter +'&quot; placeholder=&quot;First Name&quot; name=&quot;schooladminfirstname[]&quot; required />&lt;/td>';
			cols += '&lt;td>&lt;input type=&quot;text&quot; class=&quot;form-control schooladminlastname&quot; id=&quot;schooladminlastname-'+counter +'&quot; placeholder=&quot;Last Name&quot; name=&quot;schooladminlastname[]&quot; required />&lt;/td>';
			cols += '&lt;td>&lt;input type=&quot;email&quot; class=&quot;form-control schooladminemail&quot; id=&quot;schooladminemail-'+counter +'&quot; placeholder=&quot;Email&quot; name=&quot;schooladminemail[]&quot; required />&lt;/td>';

			cols += '&lt;td>&lt;i class=&quot;ibtnDel glyphicon glyphicon-remove&quot; style=&quot;padding-top: 5px;padding-left: 3px;&quot;>&lt;/i>&lt;/td>';
			newRow.append(cols);
			$(&quot;table#school_admin_table&quot;).append(newRow);
			counter++;
			schooladmincounter = counter;
			
			if(!$(&quot;table#school_admin_table&quot;).is(&quot;:visible&quot;))
			{				
				$(&quot;table#school_admin_table&quot;).show();
			}			
		});



		$(&quot;table#school_admin_table&quot;).on(&quot;click&quot;, &quot;.ibtnDel&quot;, function (event) {
			$(this).closest(&quot;tr&quot;).remove();       
			//counter -= 1
		});


	});
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
