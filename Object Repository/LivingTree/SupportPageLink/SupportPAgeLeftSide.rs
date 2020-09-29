<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>SupportPAgeLeftSide</name>
   <tag></tag>
   <elementGuidId>1e5e2691-c68d-43d5-9895-ad6982386a48</elementGuidId>
   <selectorCollection>
      <entry>
         <key>XPATH</key>
         <value>//div[@id='sidebar-wrapper']/div</value>
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
      <value>elgg-sidebar-alt</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
            

    .side-panel-green-link:link {
        color: black!important;
    }


    .side-panel-green-link:hover {
        color: #4c86c5!important;
    }


    .side-panel-green-link:active {
        color: #4c86c5!important;
    }

    .side-panel-green-link:visited {
        color: black!important;
    }

Search Members by name/email



    .side-panel-green-link:link {
        color: black!important;
    }


    .side-panel-green-link:hover {
        color: #4c86c5!important;
    }


    .side-panel-green-link:active {
        color: #4c86c5!important;
    }

    .side-panel-green-link:visited {
        color: black!important;
    }

Groups (Enabled) by name



    .side-panel-green-link:link {
        color: black!important;
    }


    .side-panel-green-link:hover {
        color: #4c86c5!important;
    }


    .side-panel-green-link:active {
        color: #4c86c5!important;
    }

    .side-panel-green-link:visited {
        color: black!important;
    }

Provisioning Provision School

    .side-panel-green-link:link {
        color: black!important;
    }


    .side-panel-green-link:hover {
        color: #4c86c5!important;
    }


    .side-panel-green-link:active {
        color: #4c86c5!important;
    }

    .side-panel-green-link:visited {
        color: black!important;
    }

Auto Provision Status

    .side-panel-green-link:link {
        color: black!important;
    }


    .side-panel-green-link:hover {
        color: #4c86c5!important;
    }


    .side-panel-green-link:active {
        color: #4c86c5!important;
    }

    .side-panel-green-link:visited {
        color: black!important;
    }





            

View All in Test Mode 

            
            



    $(&quot;#view-all-in-test-mode&quot;).click(function(e){
        e.preventDefault();
        $(&quot;.post-container&quot;).html(&quot;&lt;img src=\&quot;https://d24lt5c5b525kf.cloudfront.net/graphics_v3/ajax-loader-scroll-livingtree.gif\&quot; style=\&quot;width: 60px;margin: 0px;border: 0px;box-shadow: none;\&quot; />&quot;);
        scroll(0, 0);

        $.get(&quot;https://maple.livingtree.com/members/testmode/list&quot;, function(data, status){
               $(&quot;.post-container&quot;).html(data);
        });
    }); 




            

    .side-panel-green-link:link {
        color: black!important;
    }


    .side-panel-green-link:hover {
        color: #4c86c5!important;
    }


    .side-panel-green-link:active {
        color: #4c86c5!important;
    }

    .side-panel-green-link:visited {
        color: black!important;
    }

Import SchoolEvents
    
        
         
                Close ×
                Import Events (Work In Progress)
          
            
                    
                                        
                    
                    STEP 1: Enter Owner E-mail ID:
                     
                    
                    
                    STEP 2: Select Visibillities :
                    
                    
                    	

                    
                    STEP 3: Select CSV File:
                    
                    
                    

                    
                    STEP 4: Select Imported File: 
                    
                        Select
                    
                    
                    
                                        
                    STEP 5: Get Status  
                    
                    
                    
                    
                    
                                        
            
        
    


    .elgg-system-messages {
        z-index: 999999;
    }
    #page-content-wrapper {
	    margin-left: 215px;
	    width: 785px;
	}
	#sidebar-wrapper {
   	 margin-left: 0;
   	 z-index: auto;
   	 }
   	 #wrapper {
    padding-left:0;
	}
    



$('#get_status').click(function(){
    if($('#event_import_id').val() == ''){
    	elgg.flash_message('Please select the file name to check status.');
        return false;    
    }
	var form = $('#event_form');
    $('#event_errors').hide().val('');
    $('#event_action').val('get_status');
	
    form.ajaxSubmit(function(res){
        var result = jQuery.parseJSON(res);
        console.log(result);
        if(result['info']!='') {
            $('#event_errors').show().val('Information: \r\n' + result['info']);
        }else{
        	elgg.flash_message('&lt;span style=&quot;color:red&quot;>Some issue check with Team.&lt;/span>', 60000);
        	return false;
        }
    });        
    return false;        
});



$('#show_visibilities').click(function(){
	$('#visibilities').find('option').remove();
    if($('#owner_guid').val() == ''){
    	elgg.flash_message('Please enter the email of event owner.');
        return false;    
    }
	var form = $('#event_form');
    $('#event_errors').hide().val('');
    $('#event_action').val('get_visibilities');
    form.ajaxSubmit(function(res){
        var result = jQuery.parseJSON(res);
        console.log(result);
        if(result['error']!='') {
            $('#event_result').hide();
            if(result['errorDetails'].length > 0) {
                $('#event_errors').show().val('ERRORS: \r\n' + result['errorDetails'].join('\r\n'));
            }
            elgg.flash_message('&lt;span style=&quot;color:red&quot;>'+result['error']+'&lt;/span>', 60000);
            return false;
        }else{
        	var visibilities = result.visibilities;
        	$('#visibilities').empty();
        	for (var i = 0; i &lt; visibilities.length; i++) {
            	
            	if(visibilities[i].name == null){
            		visibilities[i].name = '';	
            	}else{
            		visibilities[i].name	=	visibilities[i].name+ ' --> ';
            	}
            	
        		$('#visibilities').append('&lt;option value=&quot;'+visibilities[i].visibility_id+'&quot; >'+visibilities[i].name+visibilities[i].visibility_name+'&lt;/option>');
        	}
        }
        //$('#event_result').hide();
        //elgg.flash_message('File Uploaded successfully, click below button to process.');
    });        
    return false;
});

$('#event_upload').click(function() {
    var form = $('#event_form');
    var visibiliites_selected  = $(&quot;#visibilities option:selected&quot;).map(function(){ return this.value }).get().join(&quot;,&quot;);
    console.log(visibiliites_selected);
    if(visibiliites_selected == ''){
    	elgg.flash_message('Please select visibilities under which these events will be mapped.');
    	return false; 
    }else{
        $('#visibiliites_selected').val(visibiliites_selected);
    }
    
    var filename = $('#event_importcsv').val();
    var file_check = /\.(csv)$/i;
    if(filename == '' || !file_check.test(filename)) {
        elgg.flash_message('&lt;span style=&quot;color:red&quot;>Please select Valid CSV file.&lt;/span>');
        return false;
    }
    //alert('File name: '+filename);
    $('#event_errors').hide().val('');
    $('#event_action').val('event_upload');
    
    form.ajaxSubmit(function(res){
        var result = jQuery.parseJSON(res);
        //console.log(result);
        $('#event_importcsv').val('');
        if(result['error']!='') {
            $('#event_result').hide();
            if(result['errorDetails'].length > 0) {
                $('#event_errors').show().val('ERRORS: \r\n' + result['errorDetails'].join('\r\n'));
            }
            elgg.flash_message('&lt;span style=&quot;color:red&quot;>'+result['error']+'&lt;/span>', 60000);
            return false;
        }
        if(result['warningDetails'].length > 0) {
            $('#event_errors').show().val('WARNINGS: \r\n' + result['warningDetails'].join('\r\n'));
        }
        $('#event_result').hide();
        $('#event_import_id').append('&lt;option value=&quot;'+result['id_import']+'&quot; selected=&quot;selected&quot;>'+result['id_import']+'-'+result['filename']+'&lt;/option>');
        elgg.flash_message('File Uploaded successfully, click below button to process.');
        return false;
    });

    return false;
});

$('#event_create_events').click(function() { 
    if($('#event_import_id').val() != '') {
        var form = $('#event_form');
        $('#event_action').val('event_create_events');
        form.ajaxSubmit(function(res){
            var result = jQuery.parseJSON(res);
            
            console.log(result);
            
            if(result['error']!='') {
                $('#event_result').hide();
                if(result['errorDetails'].length > 0) {
                    $('#event_errors').show().val('ERRORS: \r\n' + result['errorDetails'].join('\r\n'));
                }
                elgg.flash_message('&lt;span style=&quot;color:red&quot;>'+result['error']+'&lt;/span>', 60000);
                return false;
            }else{
                console.log(result['info']);
                elgg.flash_message(result['info']);
                return false;
                //elgg.flash_message('Total Event='+info.totalevents+', Successfully Created='+info.processedSuccessfully+', Having Challenges='+info.havingChallenges+'.');
            }
        });
    }
    return false;
});



$('#event_import_id').change(function() {
    $('#event_result').hide();
    $('#event_errors').hide();
});


function handleEventAction(action) {
    var form = $('#event_form');
    $('#event_action').val(action);
    form.ajaxSubmit(function(res){
        var result = jQuery.parseJSON(res);
        if(result['error']!='') {
            elgg.flash_message('&lt;span style=&quot;color:red&quot;>'+result['error']+'&lt;/span>');
            return false;
        }
        if(action == 'process') {
            $('#event_result').hide();
            elgg.flash_message(&quot;Submitted to background job for processing&quot;);
        } else if(action == 'activate_staff') {
            $('#event_result').hide();
            elgg.flash_message(&quot;Submitted to background job for activation&quot;);
        } else if(action == 'activate_all') {
            $('#event_result').hide();
            elgg.flash_message(&quot;Submitted to background job for activation&quot;);
        } else if(action == 'delete') {
            $('#import_id').find('option:selected').remove();
            $('#event_result').hide();
            elgg.flash_message(&quot;Selected Event Data Deleted Successfully&quot;);
        } else if(action == 'process_activate') {
            $('#event_result').hide();
            elgg.flash_message(&quot;Submitted to background job for processing&quot;);
        }
    });
    $('#event_errors').hide();
    return false;
}
    


    .side-panel-green-link:link {
        color: black!important;
    }


    .side-panel-green-link:hover {
        color: #4c86c5!important;
    }


    .side-panel-green-link:active {
        color: #4c86c5!important;
    }

    .side-panel-green-link:visited {
        color: black!important;
    }

Success 
						All Orgs - Weekly Report
					        </value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;sidebar-wrapper&quot;)/div[@class=&quot;elgg-sidebar-alt&quot;]</value>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:idRelative</name>
      <type>Main</type>
      <value>//div[@id='sidebar-wrapper']/div</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Logout'])[2]/following::div[12]</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Help'])[1]/following::div[12]</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//div[3]/div[6]/div/div/div/div/div</value>
   </webElementXpaths>
</WebElementEntity>
