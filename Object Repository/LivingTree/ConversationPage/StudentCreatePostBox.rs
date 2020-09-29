<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>StudentCreatePostBox</name>
   <tag></tag>
   <elementGuidId>ce3c6fea-56e2-405c-bd2a-1ff57f49c49d</elementGuidId>
   <selectorCollection>
      <entry>
         <key>XPATH</key>
         <value>//div[@id='post-component']/div/div/div[2]</value>
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
      <value>modal-footer</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
                
                    
                    Text messages are limited to 150 characters.  The subject line will not be displayed in the text message, but will appear in the conversation feed, email notifications, and mobile push notifications.
                    A phone call will be made to each recipient with a valid phone number. The message you enter will be read to the recipients. Messages will automatically translate for the recipient where applicable.
                    
                    
                        
                    
Rich Text Editor, thewire_textareaEditor toolbars Bold Italic Link Unlink Underline Insert/Remove Numbered List Insert/Remove Bulleted List Decrease Indent Increase Indent Align Left Centre Align Right JustifyPress ALT 0 for help◢ 
					
						
							
								
									
										
											
												 
												
											
										
										
											Uploading Video
										
										
											
												 
													
													
												
												
													
														
													
												
											
										
									
									
										
									
								
							
						
						
						$(document).ready(function()
						{
							$(&quot;#video_file_input_button&quot;).click(function()
							{
								$(&quot;#video_file_select_element&quot;).click();
							});
							$('#video_file_select_element').change(function()
							{
								elgg.thewire.show_tab_item('thewire',true,false, false, true);
							});
						});
						$(document).ready(function()
						{
							$(&quot;#video_tmp&quot;).on(&quot;loadeddata&quot;, function(){ 
							$(&quot;#video_screen_as_attachment&quot;).attr(&quot;src&quot;, &quot;https://maple.livingtree.com/mod/livingtree_theme/graphics_v3/film.png&quot;);

							var canvas = document.getElementById('canvas_tmp');
							var video = document.getElementById('video_tmp');
							canvas.getContext('2d').drawImage(video, 0, 0, 300, 150);

							$(&quot;#video_screen_as_attachment&quot;).attr(&quot;src&quot;, canvas.toDataURL());
							});
						});
						function capture_video_frame(file)
						{ 
							var URL = window.URL || window.webkitURL;
							var fileURL = URL.createObjectURL(file);
							$(&quot;#video_tmp&quot;).attr(&quot;src&quot;, fileURL);
							$(&quot;#video_tmp_thumb&quot;).attr(&quot;src&quot;,fileURL);
						}
						function video_upload_on_change()
						{
							$(&quot;#video_file_name_s3&quot;).val($(&quot;#video_upload_container #video_tmp_status #video_file_s3&quot;).val());
							$(&quot;#error_message&quot;).html($(&quot;#video_upload_container #video_tmp_status #video_file_upload_status_message&quot;).val());
							$(&quot;#video_file_input_button&quot;).hide();
							$(&quot;#video_upload_status&quot;).show();
							if($(&quot;#video_file_name_s3&quot;).val() == &quot;&quot;)
							{
								$(&quot;#video_upload_attachment&quot;).hide();
								if($(&quot;#error_message&quot;).html() == &quot;&quot;) 
								{
									$(&quot;#video_upload_status&quot;).show();
								} else 
								{
									$(&quot;#video_upload_status&quot;).hide();
									$(&quot;#video_file_input_button&quot;).show();
								}
							} else {
							$(&quot;#video_upload_status&quot;).hide();
							$(&quot;#video_upload_attachment&quot;).show();
							$('#thewire-submit-button').prop('disabled', false);
							}

						}
						function reset_video_fields()
						{
							if(window.jqXHR != undefined)
							{
								window.jqXHR.abort();
							}
							$(&quot;#video_screen_as_attachment&quot;).attr(&quot;src&quot;, &quot;https://maple.livingtree.com/mod/livingtree_theme/graphics_v3/film.png&quot;);
							$(&quot;#video_file_name_s3&quot;).val(&quot;&quot;);
							$(&quot;#video_upload_status&quot;).hide();
							$(&quot;#video_upload_attachment&quot;).hide();
							$(&quot;#error_message&quot;).html(&quot;&quot;);
							$('#thewire-submit-button').prop('disabled', true);
							$(&quot;#video_file_input_button&quot;).show();
						}
						$(document).ready(function ()
						{
							function reset_video_upload_tmp_holder()
							{
								$(&quot;#video_upload_container #video_tmp_status #video_file_upload_status&quot;).val(&quot;&quot;);
								$('#video_upload_container #video_tmp_status #video_file_upload_progress').val(&quot;&quot;);
								$(&quot;#video_upload_container #video_tmp_status #video_file_upload_status_message&quot;).val(&quot;&quot;);
								$(&quot;#video_upload_container #video_tmp_status #video_file_s3&quot;).val(&quot;&quot;);
							}
							function isVideo(filename) 
							{
								var ext = filename.split('.').pop();
								switch (ext.toLowerCase()) 
								{
									case 'm4v':case 'avi':case 'mpg':case 'mp4':case 'mkv':case 'wmv':case 'mov':case '3gp':case 'flv':									return true;
								}
								return false;
							}
							var form = $('#video_upload_container .direct-video-upload');
							form.fileupload(
							{
								url: &quot;https://videoinqa.s3.amazonaws.com/&quot;,
								type: form.attr('method'),
								datatype: 'json',
								add: function (event, data) 
								{
									reset_video_upload_tmp_holder();
									if(!isVideo(data.files[0].name))
									{
										$(&quot;#video_upload_container #video_tmp_status #video_file_upload_status_message&quot;).val(&quot;Unsupported file format.&quot;);
										$(&quot;#video_upload_container #video_tmp_status #video_file_upload_status&quot;).val(&quot;FileTypeUnsupported&quot;);
										video_upload_on_change();
										return FALSE;
									}
									$.ajax(
									{
										url: &quot;https://maple.livingtree.com/engine/video_s3_signature.php&quot;,
										dataType: 'json',
										async: false,
										success: function(data) 
										{
											var signature_form = '';
											$(&quot;#direct-video-upload&quot;).attr(&quot;action&quot;, data.url);
											$(&quot;#video_key_name&quot;).val(data.filename);
											$.each(data.inputs, function(key, value)
											{
												signature_form += '&lt;input type=&quot;hidden&quot; name=&quot;'+key+'&quot; value=&quot;'+value+'&quot;> ';
											}); 
											$(&quot;#form_signature&quot;).html(signature_form);
										}
									});
									window.jqXHR = data.submit();
									$(&quot;#video_upload_container #video_tmp_status #video_file_upload_status&quot;).val(&quot;Started&quot;);
									$('#video_upload_container #video_tmp_status #video_file_upload_progress').val(0);
									video_upload_on_change();
								},
								progress: function (e, data) 
								{
									var percent = Math.round((data.loaded / data.total) * 100);
									$('#video_upload_container #video_tmp_status #video_file_upload_progress').val(percent);
									$(&quot;#video_upload_container #video_tmp_status #video_file_upload_status&quot;).val(&quot;Inprogress&quot;);
									video_upload_on_change();
								},
								fail: function (e, data) 
								{ 
									$('#video_upload_container #video_tmp_status #video_file_upload_progress').val(100);
									$(&quot;#video_upload_container #video_tmp_status #video_file_upload_status&quot;).val(&quot;Error&quot;);
									video_upload_on_change();
								},
								always: function (e, data) 
								{
									var response_xml = jQuery.parseXML(data.jqXHR.responseText);
									var response_xml_obj = $(response_xml);
									var response_error_code = response_xml_obj.find(&quot;Code&quot;).html();

									var error_message = &quot;&quot;;
									switch(response_error_code) 
									{
										case 'EntityTooLarge':
											error_message = &quot;The file is too large to upload.&quot;;
											break;
										case 'AccessDenied':
											error_message = &quot;The selected file type is not supported.&quot;;
											break;
										case 'RequestTimeout':
											error_message = &quot;Network error. Video upload failed.&quot;;
											break;
										case undefined:
											error_message = &quot;Video upload failed.&quot;;
											break;
										default:
											error_message = &quot;&quot;;
									} 
									$(&quot;#video_upload_container #video_tmp_status #video_file_upload_status_message&quot;).val(error_message);
									video_upload_on_change();
								},
								done: function (event, data) 
								{
									capture_video_frame(data.files[0]);
									$(&quot;#video_upload_container #video_tmp_status #video_file_upload_status&quot;).val(&quot;Done&quot;);
									var Key = $(data.result).find(&quot;Key&quot;).text();
									$(&quot;#video_upload_container #video_tmp_status #video_file_s3&quot;).val(Key);
									video_upload_on_change();
								}
							});
						});
					
					
					
						
							
							
								
										
		
            
            
            
                
                    

                     
                
            
            
	
        
            
            
        								
							
							
								
	


	
		
	  		

	  		
	  		
		
		
	


	
	
							
						
					
					
						Add Photo
						Add Video
						Add File
																	
					
                        
                          
                        
                        I want to share this with                
                
                                               
                            Just Me                           
                            8_ALL                           
                            Aircraftinging
	
	
        	Done
        
		                                                stusbr ssbr
								
		                                                8_ALL
								
		                                                Aircraftinging
								
                
                
                
                
                
                
                
                
                
                
                
                
                
                
                
                
				
				
			
			
			
			
						
			
			
			
            
			
				
					
						                
							
							
								
								
								Disable comments
							
							
						
					
					
										
						
							
								
								
									 
									
									Schedule post for:
								
							
							
								
									
										
											
										
									
									
										
									
								
								
							
						
					
					
					
						
							
														
													
						
						
						
						 
						
						
						
						
						
						 
						
						
						
											
					
				
			

textarea {  
    width       : 500px;  
    min-height  : 50px;  
    font-family : arial,tahoma,verdana,sans-serif;  
    font-size   : 1em;  
    color       : #444;  
    padding     : 5px;
    overflow    : hidden;    
}  
.hiddendiv {  
    display     : none;  
    white-space : pre-wrap;  
    width       : 500px;  
    min-height  : 60px;  
    font-family : arial,tahoma,verdana,sans-serif;  
    font-size   : 1em;  
    padding     : 5px;  
    word-wrap   : break-word;  
    line-height : 16px;
}  
.info_text, .info_text_autodial {
    width       :100%;
    font-family : arial,tahoma,verdana,sans-serif;  
    font-size   : 1em;  
    padding     : 5px;  
    line-height : 16px;
    text-align  : justify;
    margin-bottom: 10px;
}

.info_text, .info_text_video {
    width       :100%;
    font-family : arial,tahoma,verdana,sans-serif;  
    font-size   : 1em;  
    padding     : 5px;  
    line-height : 16px;
    text-align  : justify;
    margin-bottom: 10px;
}
.create-post-bg li a{
    color:#414141!important;
    width:121px; 
    border:1px #ececec solid; 
    margin:10px 2px; /*-webkit-box-shadow: inset 0px 0px 6px 1px rgba(0, 0, 0, 0.1);*/
    padding: 8px 5px !important;
    text-align: center;
}

            


</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;post-component&quot;)/div[@class=&quot;modal-dialog&quot;]/div[@class=&quot;modal-content&quot;]/div[@class=&quot;modal-footer&quot;]</value>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:idRelative</name>
      <type>Main</type>
      <value>//div[@id='post-component']/div/div/div[2]</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Close'])[2]/following::div[1]</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Create Post'])[1]/following::div[1]</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//div[3]/div/div/div[2]</value>
   </webElementXpaths>
</WebElementEntity>
