<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>CreatePostConversationPage</name>
   <tag></tag>
   <elementGuidId>ff4ccd91-52ab-4ea8-9760-920557f343b3</elementGuidId>
   <selectorCollection>
      <entry>
         <key>XPATH</key>
         <value>//div[@id='post-component']/div/div</value>
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
      <value>modal-content</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
        
			Create Post
            Close ×
        
            
                
                    
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
																			Add Poll
												
					
                        
                          
                        
                        I want to share this with                
                
                                               
                            Just Me                           
                            walk                           
                            basavaschool
	
	
        	Done
        
		                                                teacher, tbasava.
								
		                                                Parents
								
		                                                Students
								
		                                                Classes Staff
								
		                                                Classes Parents
								
                
                
                
                
                
                
                
                
                
                
                
                
                
                
                
                
				
				
			
			
			
			
						
			
			
			
            
			
				
					
						                
							
							
								
								
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

            







var page_load_status = false;
(function($) {
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
//Added by harish for list of unsupported browser for post formatting
function load_editor()
{
	if (CKEDITOR.instances['thewire_textarea']) 
	{
	   CKEDITOR.instances['thewire_textarea'].destroy();
	}
	CKEDITOR.replace('thewire_textarea');
	$('#thewire_textarea').hide();
}

	function openEditModal(post_type, show_editor, isreshare, isreceiver, issms, isautodial, isvideo){
		if(!page_load_status){
			if(!$(&quot;.elgg-system-messages&quot;).find(&quot;.elgg-state-success&quot;).html()){
				elgg.flash_message('Loading, please wait...');
			}
			return;
		}
		//Added for reshare empty editor issue
		var editor = CKEDITOR.instances[&quot;thewire_textarea&quot;];
		if (editor) {
			editor.destroy(true);
		}
		show_everything_in_dropdown();
		if(typeof(show_editor)==='undefined') show_editor = true;
		block_help_popup=true;//block help walk through
		$('#post-component').modal(&quot;show&quot;);
		$(&quot;#post-component&quot;).find(&quot;.modal-body&quot;).show();
		$(&quot;.reshare_post&quot;).html(&quot;&quot;);
		$(&quot;#thewire_textarea&quot;).show();
		$(&quot;#thewire_textarea&quot;).val(&quot;&quot;);
		$(&quot;#thewire_textarea&quot;).removeAttr(&quot;style&quot;);
		$(&quot;#post_subject&quot;).show();
		$(&quot;#tab&quot;).show();
		$(&quot;#reshare_id&quot;).val(&quot;&quot;);
		$(&quot;#image_upload_queue&quot;).html(&quot;&quot;);
		$(&quot;#upload_error_msg&quot;).html(&quot;&quot;);
		$('.info_text_autodial').hide();
		$('.info_text_video').hide();    

		if(post_type == &quot;share-messages&quot;) {
				elgg.thewire.show_tab_item('thewire', isreceiver, issms, isautodial, isvideo);
			// CKEDITOR.replace('thewire_textarea');

			// LA-6305 - Added by Swapna for SMSALERT Functionality
			// If the issms flag is on then show sms textarea and hide the ckeditor
			if(issms &amp;&amp; issms == true)
			{
					// alert(&quot;HERE.....&quot;);
					$('.notify_chk_box').hide();
					$('#thewire_textarea').show();
					$('#thewire_textarea').attr('maxlength',150);
					$('#thewire_textarea').focus();
					$('.info_text_autodial').hide();
					$('.info_text_video').hide();
					$('.info_text').show();
					$('#thewire_sms').val(true);
			} else if (isautodial &amp;&amp; isautodial == true) 
			{
					$('.notify_chk_box').hide();
					$('#thewire_textarea').show();
					$('#thewire_textarea').attr('maxlength',2000);
					$('#thewire_textarea').focus();
					$('.info_text').hide();
					$('.info_text_autodial').show();
					$('#thewire_autodial').val(true);
					hide_everything_in_dropdown();
					limit_dropdown_to_autodial();
			} else if (isvideo &amp;&amp; isvideo == true) 
			{
					//$('.notify_chk_box').hide();
					//$('#thewire_textarea').show();
					//$('#thewire_textarea').attr('maxlength',2000);
					//$('#thewire_textarea').focus();
					$('.info_text').hide();
					//$('.info_text_video').show();
					$('#thewire_video').val(true);
					 setTimeout(function(){
					   $('.reshare_post').find('.playpause').hide();
					}, 300);
					//hide_everything_in_dropdown();
			} else if(isreceiver == false || smstabactivated == true)
			{
				$('.info_text').hide();
				$('.info_text_autodial').hide();
				$('.info_text_video').hide();
				$('#thewire_sms').val(false);
				var editor = CKEDITOR.instances[&quot;thewire_textarea&quot;];
				// Added by swapna if the ckeditor is already attached to the input element skip attaching again
				if(!editor)
				{
					load_editor();
				}
			} else if(isreceiver == false || autodialtabactivated == true)
			{
				$('.info_text').hide();
				$('.info_text_autodial').hide();
				$('#thewire_autodial').val(false);
				var editor = CKEDITOR.instances[&quot;thewire_textarea&quot;];
				if(!editor)
				{
					load_editor();
				}
			} else if(isreceiver == false || videotabactivated == true)
			{
				$('.info_text').hide();
				$('.info_text_video').hide();
				$('#thewire_video').val(false);
				var editor = CKEDITOR.instances[&quot;thewire_textarea&quot;];
				if(!editor)
				{
					load_editor();
				}
			}
		}
	}



function openModal(post_type, show_editor, isreshare, isreceiver, issms, isautodial, isvideo, isedit)
{
	elgg.thewire.edit=true;
	if(isedit != true)
	{
		elgg.thewire.edit=false;
		$('#btn_share-messages').show();
		$('#btn_share-photos').show();
		$('#btn_share-video').show();
		$('#btn_share-files').show();
		$('#btn_share-autodial').show();
		$('#btn_share-sms').show();
		if($('#btn_share-poll').length)
		{
			$('#btn_share-poll').show();
		}
		$('#edit_id').val('');
		$('#tidypics-upload-button').val('Post');
		$('#file-upload-button').val('Post');
		$('#thewire-submit-button').val('Post');
		$('#thewire-submit-button').val('Post');
		$('#divScheduledPostOptions').css('margin-top','0px');
	}
	if(!page_load_status)
	{
		if(!$(&quot;.elgg-system-messages&quot;).find(&quot;.elgg-state-success&quot;).html())
		{
			elgg.flash_message('Loading, please wait...');
		}
		return;
	}
	// check schedule post section date time enable /disable
	toggleScheduledPostDateTimeSelection();
	//Added for reshare empty editor issue
	var editor = CKEDITOR.instances[&quot;thewire_textarea&quot;];
	if (editor) 
	{
		editor.destroy(true);
	}
	show_everything_in_dropdown();
	if(typeof(show_editor)==='undefined') 
		show_editor = true;
	block_help_popup=true;//block help walk through
	$('#post-component').modal(&quot;show&quot;);
	$(&quot;#post-component&quot;).find(&quot;.modal-body&quot;).show();
	$(&quot;.reshare_post&quot;).html(&quot;&quot;);
	$(&quot;#thewire_textarea&quot;).show();
	$(&quot;#thewire_textarea&quot;).val(&quot;&quot;);
	$(&quot;#thewire_textarea&quot;).removeAttr(&quot;style&quot;);
	$(&quot;#post_subject&quot;).show();
	$(&quot;#tab&quot;).show();
	$(&quot;#reshare_id&quot;).val(&quot;&quot;);
	$(&quot;#image_upload_queue&quot;).html(&quot;&quot;);
	$(&quot;#upload_error_msg&quot;).html(&quot;&quot;);
	$('.info_text_autodial').hide();
	$('.info_text_video').hide();    

	if(post_type == &quot;share-messages&quot;) 
	{
		elgg.thewire.show_tab_item('thewire', isreceiver, issms, isautodial, isvideo);
		if(issms &amp;&amp; issms == true)
		{
			$('.notify_chk_box').hide();
			$('#thewire_textarea').show();
			$('#thewire_textarea').attr('maxlength',150);
			$('#thewire_textarea').focus();
			$('.info_text_autodial').hide();
			$('.info_text_video').hide();
			$('.info_text').show();
			$('#thewire_sms').val(true);
		}
		else if(isautodial &amp;&amp; isautodial == true) 
		{
			$('.notify_chk_box').hide();
			$('#thewire_textarea').show();
			$('#thewire_textarea').attr('maxlength',2000);
			$('#thewire_textarea').focus();
			$('.info_text').hide();
			$('.info_text_autodial').show();
			$('#thewire_autodial').val(true);
			hide_everything_in_dropdown();
			limit_dropdown_to_autodial();
		} 
		else if (isvideo &amp;&amp; isvideo == true) 
		{
			$('.info_text').hide();
			$('#thewire_video').val(true);
			setTimeout(function()
			{
			   $('.reshare_post').find('.playpause').hide();
			}, 300);
		}
		else if(isreceiver == false || smstabactivated == true)
		{
			$('.info_text').hide();
			$('.info_text_autodial').hide();
			$('.info_text_video').hide();
			$('#thewire_sms').val(false);
			var editor = CKEDITOR.instances[&quot;thewire_textarea&quot;];
			if(!editor)
			{
				load_editor();
			}
		}
		else if(isreceiver == false || autodialtabactivated == true)
		{
			$('.info_text').hide();
			$('.info_text_autodial').hide();
			$('#thewire_autodial').val(false);
			var editor = CKEDITOR.instances[&quot;thewire_textarea&quot;];
			if(!editor)
			{
				load_editor();
			}
		}
		else if(isreceiver == false || videotabactivated == true)
		{
			$('.info_text').hide();
			$('.info_text_video').hide();
			$('#thewire_video').val(false);
			var editor = CKEDITOR.instances[&quot;thewire_textarea&quot;];
			if(!editor)
			{
				load_editor();
			}
		}
	}
	else if(post_type == &quot;share-photos&quot;)
	{
		elgg.thewire.show_tab_item('photo', isreceiver);
		$('#thewire_sms').val(false);
		$('#thewire_autodial').val(false);
		$('#thewire_video').val(false);
		if(isreceiver == false || smstabactivated == true)
		{
			$('.info_text').hide();
			$('.info_text_autodial').hide();
			$('.info_text_video').hide();
			var editor = CKEDITOR.instances[&quot;thewire_textarea&quot;];
			if(!editor)
			{
				load_editor();
			}
		}
		else if(isreceiver == false || autodialtabactivated == true)
		{
			$('.info_text').hide();
			$('.info_text_autodial').hide();
			var editor = CKEDITOR.instances[&quot;thewire_textarea&quot;];
			if(!editor)
			{
				load_editor();
			}
		}
		else if(isreceiver == false || videotabactivated == true)
		{
			$('.info_text').hide();
			$('.info_text_video').hide();
			var editor = CKEDITOR.instances[&quot;thewire_textarea&quot;];
			if(!editor)
			{
				load_editor();
			}
		}
	}
	else if(post_type == &quot;share-files&quot;)
	{
		$('#thewire_sms').val(false);
		$('#thewire_autodial').val(false);
		$('#thewire_video').val(false);
		if(navigator.userAgent.match(/iPad/i) == &quot;iPad&quot; &amp;&amp; navigator.userAgent.match(/5.1 Mobile/i) == &quot;5.1 Mobile&quot;)
		{
			elgg.thewire.show_tab_item('thewire',isreceiver);
			$('#error_message').text(&quot;File post is disabled for this device&quot;);
			$(&quot;#btn_share-files&quot;).hide();
		}
		else
		{
			elgg.thewire.show_tab_item('file', isreceiver);
			if(isreceiver == false || smstabactivated == true) 
			{
				$('.info_text').hide();
				$('.info_text_autodial').hide();
				$('.info_text_video').hide();
				var editor = CKEDITOR.instances[&quot;thewire_textarea&quot;];
				if(!editor)
				{
					load_editor();
				}
			}
			else if(isreceiver == false || autodialtabactivated == true) 
			{
				$('.info_text').hide();
				$('.info_text_autodial').hide();
				var editor = CKEDITOR.instances[&quot;thewire_textarea&quot;];
				if(!editor)
				{
					load_editor();
				}
			}
			else if(isreceiver == false || videotabactivated == true) 
			{
				$('.info_text').hide();
				$('.info_text_video').hide();
				var editor = CKEDITOR.instances[&quot;thewire_textarea&quot;];
				if(!editor)
				{
					load_editor();
				}
			}
		}
	}
	else if(post_type == &quot;share-poll&quot;)
	{
		$('#thewire_sms').val(false);
		$('#thewire_autodial').val(false);
		$('#thewire_video').val(false);
		elgg.thewire.show_tab_item('poll', isreceiver);
		if(isreceiver == false || smstabactivated == true) 
		{
			$('.info_text').hide();
			$('.info_text_autodial').hide();
			$('.info_text_video').hide();
			var editor = CKEDITOR.instances[&quot;thewire_textarea&quot;];
			if(!editor)
			{
				load_editor();
			}
		}
		else if(isreceiver == false || autodialtabactivated == true) 
		{
			$('.info_text').hide();
			$('.info_text_autodial').hide();
			var editor = CKEDITOR.instances[&quot;thewire_textarea&quot;];
			if(!editor)
			{
				load_editor();
			}
		}
		else if(isreceiver == false || videotabactivated == true) 
		{
			$('.info_text').hide();
			$('.info_text_video').hide();
			var editor = CKEDITOR.instances[&quot;thewire_textarea&quot;];
			if(!editor)
			{
				load_editor();
			}
		}
	}
	if(typeof(isreshare)==true)
	{
		CKEDITOR.instances.thewire_textarea.setData(&quot;&quot;);
	}

	// Edit post 
	if(typeof(isedit)==true)
	{
		if(isvideo &amp;&amp; isvideo == true)
		{

		}
		else if (isautodial &amp;&amp; isautodial == true)
		{

		}
		else
		{
			CKEDITOR.instances.thewire_textarea.setData(&quot;&quot;);
		}
	}
	if (isautodial &amp;&amp; isautodial == true)
	{
		$('#thewire_textarea').hide();
	}
	if (isvideo &amp;&amp; isvideo == true) 
	{
	}
	isreshare = false;
	isreceiver=false;
	issms = false; // Reset the issms flag
	isautodial = false;
	isvideo = false;
}

function openModalEmergencyDefault(){
	if(!page_load_status)
	{
		if(!$(&quot;.elgg-system-messages&quot;).find(&quot;.elgg-state-success&quot;).html())
		{
			elgg.flash_message('Loading, please wait...');
		}
		return;
	}

	// check for user emergency msg already exists or not
	$.ajax(
	{
		url: &quot;https://maple.livingtree.com/action/thewire/get_default_emergency_message&quot;,
		type: &quot;post&quot;,
		async: false,
		success: function(data) 
		{
			data = $.parseJSON(data)
			if(data.output){
				// show emergency default message
				res = data.output;
				res = res.split(&quot;~&quot;);
				$('#emergency_default_modal_subject').val(res[0]);
				$('#emergency_default_modal_text').val(res[1]);
			}
			else{	
				// open default popup		
				$('#emergency_default_modal_subject').val(&quot;&quot;);
				$('#emergency_default_modal_text').val(&quot;&quot;);
				$('#error_message_emergency').text(&quot;&quot;);
			}
		}
	});

	$('#emergency_default_modal').modal(&quot;show&quot;);
}

function save_default_emergency_message(){
	if($('#emergency_default_modal_text').val() == &quot;&quot;){
		$('#error_message_emergency').text(&quot;Please write some text to save&quot;);
		return;
	}
	$.ajax(
	{
		url: &quot;https://maple.livingtree.com/action/thewire/save_default_emergency_message&quot;,
		type: &quot;post&quot;,
		data: { 'title': $('#emergency_default_modal_subject').val(), 'description' : $('#emergency_default_modal_text').val() },
		async: false,
		success: function(data) 
		{
			data = $.parseJSON(data)
			if(data.output){
				// close the emergency default message popup
				$('#emergency_default_modal').modal('hide');
			}
			else{			
				$('#error_message_emergency').text(&quot;Error while saving default message. Please contact support team.&quot;);
			}
		}
	});
}

function openModalEmergency(post_type, show_editor, isreshare, isreceiver, issms, isautodial, isvideo, isedit,isemergency)
{
	elgg.thewire.edit=true;
	if(isedit != true)
	{
		elgg.thewire.edit=false;
		$('#btn_share-messages').show();
		$('#btn_share-photos').show();
		$('#btn_share-video').show();
		$('#btn_share-files').show();
		$('#btn_share-autodial').show();
		$('#btn_share-sms').show();
		if($('#btn_share-poll').length)
		{
			$('#btn_share-poll').show();
		}
		$('#edit_id').val('');
		$('#tidypics-upload-button').val('Post');
		$('#file-upload-button').val('Post');
		$('#thewire-submit-button').val('Post');
		$('#thewire-submit-button').val('Post');
		$('#divScheduledPostOptions').css(&quot;display&quot;, &quot;block&quot;);
		$('#divScheduledPostOptions').css('margin-top','0px');
	}
	if(!page_load_status)
	{
		if(!$(&quot;.elgg-system-messages&quot;).find(&quot;.elgg-state-success&quot;).html())
		{
			elgg.flash_message('Loading, please wait...');
		}
		return;
	}
	// check schedule post section date time enable /disable
	toggleScheduledPostDateTimeSelection();
	//Added for reshare empty editor issue
	var editor = CKEDITOR.instances[&quot;thewire_textarea&quot;];
	if (editor) 
	{
		editor.destroy(true);
	}
	show_everything_in_dropdown();
	if(typeof(show_editor)==='undefined') 
		show_editor = true;
	block_help_popup=true;//block help walk through
	$(&quot;#thewire_emergency&quot;).val(&quot;true&quot;);

	// check for user emergency msg already exists or not
	$.ajax(
	{
		url: &quot;https://maple.livingtree.com/action/thewire/get_default_emergency_message&quot;,
		type: &quot;post&quot;,
		async: false,
		success: function(data) 
		{
			data = $.parseJSON(data)
			if(data.output){
				// show emergency default message
				res = data.output;
				res = res.split(&quot;~&quot;);
				$('#post_subject').val(res[0]);
				$('#thewire_textarea').val(res[1]);
			}
			else{	
				// open default popup		
				$('#post_subject').val(&quot;&quot;);
				$('#thewire_textarea').val(&quot;&quot;);
			}
		}
	});
	$('.comment_chk_box > input').prop('checked', true);

	$('#post-component').modal(&quot;show&quot;);
	$(&quot;#post-component&quot;).find(&quot;.modal-body&quot;).show();

	$(&quot;#nocomment&quot;).val(&quot;nocomment&quot;);
	
	$(&quot;.reshare_post&quot;).html(&quot;&quot;);
	$(&quot;#thewire_textarea&quot;).show();
	//$(&quot;#thewire_textarea&quot;).val(&quot;&quot;);
	$(&quot;#thewire_textarea&quot;).removeAttr(&quot;style&quot;);
	$(&quot;#post_subject&quot;).show();
	$(&quot;#tab&quot;).show();
	$(&quot;#reshare_id&quot;).val(&quot;&quot;);
	$(&quot;#image_upload_queue&quot;).html(&quot;&quot;);
	$(&quot;#upload_error_msg&quot;).html(&quot;&quot;);
	$('.info_text_autodial').hide();
	$('.info_text_video').hide();


	// code for showing staff and parent group selected by default
	/*$(&quot;.chzn_lt-choices&quot;).append('&lt;li class=&quot;search-choice&quot; id=&quot;the_wire_post_'+main_key+'&quot;>&lt;a href=&quot;javascript:void(0)&quot; class=&quot;search-choice-close&quot; rel=&quot;'+post_val+'&quot;>x&lt;/a>&lt;span>'+main_text+'&lt;/span>&lt;/li>');
    alert(7)
    add_selected_tick_mark(&quot;#opt_text_&quot;+main_key, main_key);*/
    /*$(&quot;div.post_options_temp_sub ul li&quot;).each(function(index){
    	console.log(index + &quot;: &quot; + $( this ).html());

    }) */
    // end   

	if(post_type == &quot;share-messages&quot;) 
	{
		elgg.thewire.show_tab_item_emergency('thewire', isreceiver, issms, isautodial, isvideo,isemergency);

		if(issms &amp;&amp; issms == true)
		{
			$('.notify_chk_box').hide();
			$('#thewire_textarea').show();
			$('#thewire_textarea').attr('maxlength',150);
			$('#thewire_textarea').focus();
			$('.info_text_autodial').hide();
			$('.info_text_video').hide();
			$('.info_text').show();
			$('#thewire_sms').val(true);
		}
		else if(isautodial &amp;&amp; isautodial == true) 
		{
			$('.notify_chk_box').hide();
			$('#thewire_textarea').show();
			$('#thewire_textarea').attr('maxlength',2000);
			$('#thewire_textarea').focus();
			$('.info_text').hide();
			$('.info_text_autodial').show();
			$('#thewire_autodial').val(true);
			hide_everything_in_dropdown();
			limit_dropdown_to_autodial();
		} 
		else if(isemergency &amp;&amp; isemergency == true) 
		{
			$('.notify_chk_box').hide();
			$('#thewire_textarea').show();
			$('#thewire_textarea').attr('maxlength',160);
			$('#thewire_textarea').focus();
			$('.info_text').hide();
			$('.info_text_autodial').show();
			$('#thewire_autodial').val(true);
			$('#thewire_sms').val(true);
			hide_everything_in_dropdown();
			limit_dropdown_to_autodial();

			// to be written in some function in mod/livingtree_theme/views/default/js/popup.php
			$(&quot;#nav.elgg-tag-groups-list li&quot;).each(function(index){
				if($( this ).css('display') != &quot;none&quot;){
					group_id = $( this ).attr(&quot;data-group_id&quot;);	
					//app_saved_tags = [];				
					$(&quot;div#temp_post_options_&quot;+group_id+&quot; ul li span&quot;).each(function(index){
						span_text = $( this ).html();
						if((span_text.indexOf(&quot;Parents&quot;) != -1 || span_text.indexOf(&quot;Staff&quot;) != -1) &amp;&amp; span_text.indexOf(&quot;Students&quot;) == -1){
						    //console.log(span_text);
						    li_attr_rel_str = $( this ).attr(&quot;data-visibility_info&quot;);
						    li_attr_rel_arr = li_attr_rel_str.split(&quot;|&quot;);
						    dialog_text = $( this ).attr(&quot;data-group_name&quot;);
						    sub_text = $( this ).text();
						    sub_text = processPostOptionSelection( sub_text );
						    if(dialog_text == sub_text){ //if visibility name is same as group name then use the group name
			                    main_text = dialog_text;
			                } else {
			                    main_text = dialog_text+&quot; - &quot;+sub_text;
			                }
			                
						    //app_saved_tags.push(li_attr_rel_str);
						    //app_saved_post_options = app_saved_tags;
						    //$(&quot;#thewire_post_options&quot;).val(app_saved_post_options);
						    //console.log(app_saved_tags)

						    if(app_saved_tags.indexOf(li_attr_rel_str) === -1){
						    	app_saved_tags.push(li_attr_rel_str);
						    	app_saved_post_options = app_saved_tags;
						    	$(&quot;#thewire_post_options&quot;).val(app_saved_post_options);
						    	$(&quot;.chzn_lt-choices&quot;).append('&lt;li class=&quot;search-choice&quot; id=&quot;the_wire_post_'+li_attr_rel_arr[1]+'&quot;>&lt;a href=&quot;javascript:void(0)&quot; class=&quot;search-choice-close&quot; rel=&quot;'+li_attr_rel_str+'&quot;>x&lt;/a>&lt;span>'+main_text+'&lt;/span>&lt;/li>');
						    		add_selected_tick_mark(&quot;#opt_text_&quot;+li_attr_rel_arr[1], li_attr_rel_arr[1]);
						    		$(&quot;#the_wire_post_types_chzn_lt_modified&quot;).css('display','block');
						    }

						    //$(&quot;.chzn_lt-choices&quot;).append('&lt;li class=&quot;search-choice&quot; id=&quot;the_wire_post_'+li_attr_rel_arr[1]+'&quot;>&lt;a href=&quot;javascript:void(0)&quot; class=&quot;search-choice-close&quot; rel=&quot;'+li_attr_rel_str+'&quot;>x&lt;/a>&lt;span>'+main_text+'&lt;/span>&lt;/li>');
						    //add_selected_tick_mark(&quot;#opt_text_&quot;+li_attr_rel_arr[1], li_attr_rel_arr[1]);
						    //$(&quot;#the_wire_post_types_chzn_lt_modified&quot;).css('display','block');
						}
					});
				}
		    	
		    });
		    // to be written in some function in mod/livingtree_theme/views/default/js/popup.php

		} 
		else if (isvideo &amp;&amp; isvideo == true) 
		{
			$('.info_text').hide();
			$('#thewire_video').val(true);
			setTimeout(function()
			{
			   $('.reshare_post').find('.playpause').hide();
			}, 300);
		}
		else if(isreceiver == false || smstabactivated == true)
		{
			$('.info_text').hide();
			$('.info_text_autodial').hide();
			$('.info_text_video').hide();
			$('#thewire_sms').val(false);
			var editor = CKEDITOR.instances[&quot;thewire_textarea&quot;];
			if(!editor)
			{
				load_editor();
			}
		}
		else if(isreceiver == false || autodialtabactivated == true)
		{
			$('.info_text').hide();
			$('.info_text_autodial').hide();
			$('#thewire_autodial').val(false);
			var editor = CKEDITOR.instances[&quot;thewire_textarea&quot;];
			if(!editor)
			{
				load_editor();
			}
		}
		else if(isreceiver == false || videotabactivated == true)
		{
			$('.info_text').hide();
			$('.info_text_video').hide();
			$('#thewire_video').val(false);
			var editor = CKEDITOR.instances[&quot;thewire_textarea&quot;];
			if(!editor)
			{
				load_editor();
			}
		}
		

	}
	else if(post_type == &quot;share-photos&quot;)
	{
		/*elgg.thewire.show_tab_item('photo', isreceiver);
		$('#thewire_sms').val(false);
		$('#thewire_autodial').val(false);
		$('#thewire_video').val(false);
		if(isreceiver == false || smstabactivated == true)
		{
			$('.info_text').hide();
			$('.info_text_autodial').hide();
			$('.info_text_video').hide();
			var editor = CKEDITOR.instances[&quot;thewire_textarea&quot;];
			if(!editor)
			{
				load_editor();
			}
		}
		else if(isreceiver == false || autodialtabactivated == true)
		{
			$('.info_text').hide();
			$('.info_text_autodial').hide();
			var editor = CKEDITOR.instances[&quot;thewire_textarea&quot;];
			if(!editor)
			{
				load_editor();
			}
		}
		else if(isreceiver == false || videotabactivated == true)
		{
			$('.info_text').hide();
			$('.info_text_video').hide();
			var editor = CKEDITOR.instances[&quot;thewire_textarea&quot;];
			if(!editor)
			{
				load_editor();
			}
		}*/
	}
	else if(post_type == &quot;share-files&quot;)
	{
		/*$('#thewire_sms').val(false);
		$('#thewire_autodial').val(false);
		$('#thewire_video').val(false);
		if(navigator.userAgent.match(/iPad/i) == &quot;iPad&quot; &amp;&amp; navigator.userAgent.match(/5.1 Mobile/i) == &quot;5.1 Mobile&quot;)
		{
			elgg.thewire.show_tab_item('thewire',isreceiver);
			$('#error_message').text(&quot;File post is disabled for this device&quot;);
			$(&quot;#btn_share-files&quot;).hide();
		}
		else
		{
			elgg.thewire.show_tab_item('file', isreceiver);
			if(isreceiver == false || smstabactivated == true) 
			{
				$('.info_text').hide();
				$('.info_text_autodial').hide();
				$('.info_text_video').hide();
				var editor = CKEDITOR.instances[&quot;thewire_textarea&quot;];
				if(!editor)
				{
					load_editor();
				}
			}
			else if(isreceiver == false || autodialtabactivated == true) 
			{
				$('.info_text').hide();
				$('.info_text_autodial').hide();
				var editor = CKEDITOR.instances[&quot;thewire_textarea&quot;];
				if(!editor)
				{
					load_editor();
				}
			}
			else if(isreceiver == false || videotabactivated == true) 
			{
				$('.info_text').hide();
				$('.info_text_video').hide();
				var editor = CKEDITOR.instances[&quot;thewire_textarea&quot;];
				if(!editor)
				{
					load_editor();
				}
			}
		}*/
	}
	else if(post_type == &quot;share-poll&quot;)
	{
		/*$('#thewire_sms').val(false);
		$('#thewire_autodial').val(false);
		$('#thewire_video').val(false);
		elgg.thewire.show_tab_item('poll', isreceiver);
		if(isreceiver == false || smstabactivated == true) 
		{
			$('.info_text').hide();
			$('.info_text_autodial').hide();
			$('.info_text_video').hide();
			var editor = CKEDITOR.instances[&quot;thewire_textarea&quot;];
			if(!editor)
			{
				load_editor();
			}
		}
		else if(isreceiver == false || autodialtabactivated == true) 
		{
			$('.info_text').hide();
			$('.info_text_autodial').hide();
			var editor = CKEDITOR.instances[&quot;thewire_textarea&quot;];
			if(!editor)
			{
				load_editor();
			}
		}
		else if(isreceiver == false || videotabactivated == true) 
		{
			$('.info_text').hide();
			$('.info_text_video').hide();
			var editor = CKEDITOR.instances[&quot;thewire_textarea&quot;];
			if(!editor)
			{
				load_editor();
			}
		}*/
	}
	if(typeof(isreshare)==true)
	{
		CKEDITOR.instances.thewire_textarea.setData(&quot;&quot;);
	}

	// Edit post 
	if(typeof(isedit)==true)
	{
		if(isvideo &amp;&amp; isvideo == true)
		{

		}
		else if (isautodial &amp;&amp; isautodial == true)
		{

		}
		else
		{
			CKEDITOR.instances.thewire_textarea.setData(&quot;&quot;);
		}
	}
	if (isautodial &amp;&amp; isautodial == true)
	{
		$('#thewire_textarea').hide();
	}
	if (isvideo &amp;&amp; isvideo == true) 
	{
	}
	isreshare = false;
	isreceiver=false;
	issms = false; // Reset the issms flag
	isautodial = false;
	isvideo = false;
}
	
$('#post-component').on('hidden.bs.modal', function () {
    var editor = CKEDITOR.instances[&quot;thewire_textarea&quot;];
    // Added by swapna if the ckeditor is already attached to the input element skip attaching again
    if (editor) {
        editor.destroy(true);
    }
    $(&quot;.reshare_post&quot;).html(&quot;&quot;);
    $(&quot;#thewire_textarea&quot;).val(&quot;&quot;);
    $(&quot;#thewire_textarea&quot;).removeAttr(&quot;style&quot;);
    $(&quot;#post_subject&quot;).val(&quot;&quot;);
    $(&quot;#reshare_id&quot;).val(&quot;&quot;);
    $(&quot;#image_upload_queue&quot;).html(&quot;&quot;);
    $(&quot;#temp_file_name&quot;).html(&quot;&quot;);
    $(&quot;#upload_error_msg&quot;).html(&quot;&quot;);
    $(&quot;#enclose_post_opts&quot;).hide();
    remove_all_tick_mark();
    $('#error_message').text(&quot;&quot;);
    $(&quot;#choice_mdf&quot;).html(&quot;&quot;);
    $(&quot;#thewire_photo_tags&quot;).val(&quot;&quot;);
    $(&quot;#thewire_post_options&quot;).val(&quot;&quot;);
    $(&quot;#thewire_album_related_groupids&quot;).val(&quot;&quot;);
    $(&quot;#post_settings&quot;).find(&quot;input[type=checkbox]&quot;).removeAttr(&quot;checked&quot;);
    $(&quot;#nocomment&quot;).val(&quot;&quot;);
    $(&quot;#reshare&quot;).val(&quot;&quot;);
    $(&quot;#important&quot;).val(&quot;&quot;);
    $(&quot;#hd_deleted_photos&quot;).remove();
    $(&quot;#hd_cancel_photos&quot;).remove();
    deleted_photos = [];
    $(&quot;#hdbatch_guid&quot;).val(&quot;&quot;);
    $(&quot;#btn_share-files&quot;).removeClass(&quot;active&quot;);
    $(&quot;#btn_share-photos&quot;).removeClass(&quot;active&quot;)
    app_saved_tags = [];
    app_saved_post_options = [];
    $('#thewire_textarea').hide();

    $(&quot;#thewire_emergency&quot;).val(&quot;false&quot;);

    //Recreating post for IE11 to fix issues with blank post issues when we open reshare and close.
    if (BrowserDetect.browser == &quot;Explorer&quot;){
        var ver = getInternetExplorerVersion();
        if(ver==11){
            var post_content = $(&quot;#main-div-ajax-call&quot;).html();
            $(&quot;#main-div-ajax-call&quot;).html(&quot;&quot;);
            $(&quot;#main-div-ajax-call&quot;).html(post_content);
        }
    }
});
$( window ).load(function() {
    page_load_status = true;
});



var check_autodial_text_count = function(){

	if($('#autodial_text').val().length >= 2000){
		$(&quot;#error_message&quot;).html(&quot;The message length should not exceed 2000 characters&quot;);
	}

}

var check_video_text_count = function(){

	if($('#video_text').val().length >= 2000){
		$(&quot;#error_message&quot;).html(&quot;The message length should not exceed 2000 characters&quot;);
	}

}


$( document ).ready(function() {
	var txt_autodial = $('#autodial_text');

	txt_autodial.on('keyup',function(){
	  var txt = $('#thewire_textarea');
	  txt.val($('#autodial_text').val());
	  txt.keyup();
          check_autodial_text_count();
	});

	txt_autodial.on('keydown',function(){
	  var txt = $('#thewire_textarea');
	  txt.val($('#autodial_text').val());
	  txt.keydown();
          check_autodial_text_count();
	});

	txt_autodial.on('change',function(){
	  var txt = $('#thewire_textarea');
	  txt.val($('#autodial_text').val());
	  txt.change();
          check_autodial_text_count();
	});

	txt_autodial.on('blur',function(){
	  var txt = $('#thewire_textarea');
	  txt.val($('#autodial_text').val());
	  txt.blur();
          check_autodial_text_count();
	});

	txt_autodial.on('focus',function(){
	  var txt = $('#thewire_textarea');
	  txt.val($('#autodial_text').val());
	  txt.focus();
          check_autodial_text_count();
	});
        
        
        
        
        
        
 
	var txt_video = $('#video_text');

	txt_video.on('keyup',function(){
	  var txt = $('#thewire_textarea');
	  txt.val($('#video_text').val());
	  txt.keyup();
          check_video_text_count();
	});

	txt_video.on('keydown',function(){
	  var txt = $('#thewire_textarea');
	  txt.val($('#video_text').val());
	  txt.keydown();
          check_video_text_count();
	});

	txt_video.on('change',function(){
	  var txt = $('#thewire_textarea');
	  txt.val($('#video_text').val());
	  txt.change();
          check_video_text_count();
	});

	txt_video.on('blur',function(){
	  var txt = $('#thewire_textarea');
	  txt.val($('#video_text').val());
	  txt.blur();
          check_video_text_count();
	});

	txt_video.on('focus',function(){
	  var txt = $('#thewire_textarea');
	  txt.val($('#video_text').val());
	  txt.focus();
          check_video_text_count();
	});
	
	$('.modal-footer').on('click', '#add-poll-answer', function ()
	{
        addPollAnswer();
    });
	$('.modal-footer').on('click', '#poll-answers li .remove', function ()
	{
        removePollAnswer($(this));
    });
});


</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;post-component&quot;)/div[@class=&quot;modal-dialog&quot;]/div[@class=&quot;modal-content&quot;]</value>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:idRelative</name>
      <type>Main</type>
      <value>//div[@id='post-component']/div/div</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Close'])[1]/following::div[6]</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Default Urgent Alert Message'])[1]/following::div[6]</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//fieldset/div[3]/div/div</value>
   </webElementXpaths>
</WebElementEntity>
