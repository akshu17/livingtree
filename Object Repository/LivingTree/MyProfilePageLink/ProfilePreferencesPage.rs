<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>ProfilePreferencesPage</name>
   <tag></tag>
   <elementGuidId>9990961a-bf76-48e6-862d-8bb5bd53b3ad</elementGuidId>
   <selectorCollection>
      <entry>
         <key>XPATH</key>
         <value>//form[@id='editprofilefrm']/fieldset</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>fieldset</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
/*
 * 
 * LA-6438: SMS Integration:  User opt-in process for receiving texts; required by carriers
 * Purpose: reset the sms alert flag if the phone number changes
 * @returns none
 */
function resetsmson() {
    var new_phone = $('input[type=&quot;text&quot;][name=&quot;phone_view&quot;]').val();
    var country = $('#country_select').val();
    var new_phone_local = formatE164(country, new_phone).replace('+','');
    old_phone_number = old_phone_number.replace('+','');
    
    // phone number changed then revalidate the phone number
    if(new_phone_local != old_phone_number &amp;&amp; sms_on_flag == &quot;yes&quot;) {
        $('input[type=&quot;radio&quot;][name=&quot;sms_on&quot;][value=&quot;no&quot;]').attr('checked', true);
        $('.error_message_phone_number').text('Phone number has to be re-validated to enable the SMS alerts.');
        return;
    }
    else if(new_phone_local == old_phone_number &amp;&amp; sms_on_flag == &quot;yes&quot;) {
        $('input[type=&quot;radio&quot;][name=&quot;sms_on&quot;][value=&quot;yes&quot;]').attr('checked', true);
        $('.error_message_phone_number').text('');
        return;
    }
    else if(new_phone_local == old_phone_number &amp;&amp; sms_on_flag == &quot;no&quot;) {
        $('input[type=&quot;radio&quot;][name=&quot;sms_on&quot;][value=&quot;no&quot;]').attr('checked', true); 
        return;
    }
}
/*
 * 
 * LA-6438: SMS Integration:  User opt-in process for receiving texts; required by carriers
 * Purpose: enable OK button when user starts typing the activation code
 * @returns none
 */
function enableokbutton() {
    $(&quot;#validateactivationcode&quot;).removeAttr(&quot;disabled&quot;);    
}
/*
 * 
 * LA-6438: SMS Integration:  User opt-in process for receiving texts; required by carriers
 * Purpose: Validate the activation code(GUID) provided by the user with the user's GUID
 * @returns none
 */
function validatetheactivationcode() {
    var guid=$('input[name=&quot;guid&quot;]').val();
    var phone = $('input[name=&quot;phone&quot;]').val();
    var activation_code = $('#activation_code').val();
    var activation='validate';
    var country = $('#country_select').val();
    
    if(activation_code == ''){
        $('.error_activation_code').css('color','red');
        $('.modal-footer').css('margin-top','30px');
        $('.error_activation_code').text('Please enter the activation code.');
        return;
    }
    else if(activation_code != guid) {
        $('.error_activation_code').css('color','red');
        $('.modal-footer').css('margin-top','30px');
        $('.error_activation_code').text('The activation code is invalid.');
        return;
    }
    
    
    $.ajax({type: &quot;POST&quot;,
        url: &quot;https://maple.livingtree.com/actions/profile/activationcode.php&quot;,
        dataType: &quot;html&quot;,
        cache: false,
        data: {activation:activation,activation_code: activation_code, guid: guid,phone:phone,country:country},
        success: function(htmlData) {
            if(htmlData == 1)
            {
                // alert($(&quot;#validateactivationcode&quot;));
                old_phone_number = $('input[name=&quot;phone&quot;]').val().replace('+','');
                // alert(old_phone_number);
                sms_on_flag = 'yes';
                $('#validatephonenumModal').modal('hide');
                $('#errorModal').modal('hide');
                $('.error_activation_code').text('');
                $('.error_message_phone_number').text('');
                $('input[type=&quot;radio&quot;][name=&quot;sms_on&quot;][value=&quot;yes&quot;]').attr('checked', true);
                elgg.flash_message('Phone verification completed successfully.');
            }
            else
            {
                $('.error_activation_code').css('color','red');
                $('.modal-footer').css('margin-top','30px');
                $('.error_activation_code').text('Invalid Activation Code.');
            }
        }
    });
}
/*
 * 
 * LA-6438: SMS Integration:  User opt-in process for receiving texts; required by carriers
 * Purpose: Send the activation code to the user's phone number
 * @returns none
 */
function sendactivationcode()
{
	var guid=$('input[name=&quot;guid&quot;]').val();
	var phone = $('input[name=&quot;phone&quot;]').val();
	var country = $('#country_select').val();
	//alert($('input[name=&quot;guid&quot;]').val());
	//alert($('input[name=&quot;phone&quot;]').val());
	var activation='send';
	
	$.ajax({type: &quot;POST&quot;,
		url: &quot;https://maple.livingtree.com/actions/profile/activationcode.php&quot;,
		dataType: &quot;html&quot;,
		cache: false,
		data: {activation:activation,guid: guid,phone:phone,country:country},
		success: function(htmlData) {
				//alert(&quot;Ajax Response:&quot;+htmlData);
				if(htmlData == 1)
				{
					//alert($(&quot;#validateactivationcode&quot;));
					$('.error_activation_code').css('color','blue');
					$('.modal-footer').css('margin-top','30px');
					$('.error_activation_code').html(&quot;An activation code has been sent to your mobile number. Please enter the code above and click OK.&quot;);
					$(&quot;#validateactivationcode&quot;).removeAttr(&quot;disabled&quot;);
					//alert($('#btnsendcode').text());
					$('#btnsendcode').text('Resend Code');
				}
		}
	});
}

function resetvalidate_phone_number_popup() {
    $('#activation_code').val('');
    $('.error_activation_code').text('');
    $(&quot;#validateactivationcode&quot;).attr(&quot;disabled&quot;,'true');
}
/*
 * 
 * LA-6438: SMS Integration:  User opt-in process for receiving texts; required by carriers
 * Purpose: Return from the error message displayed for not a valid phone number
 * @returns none
 */
function cancel_validate_phone_number() {
    $('#validatephonenumModal').modal('hide');
    $('#errorModal').modal('hide');
    resetsmson();
    resetvalidate_phone_number_popup();
}
/*
 * 
 * LA-6438: SMS Integration:  User opt-in process for receiving texts; required by carriers
 * Purpose: Validate the phone numbers if the user preference for &quot;Allow text message posts to be sent to my phone&quot; is set to yes
 * @returns none
 */
function validate_phone_number() {
    var phone = $('input[type=&quot;text&quot;][name=&quot;phone_view&quot;]');
    phone_val= phone.val();
    var phonenumber = phone_val.replace('+','');
    var country = $('#country_select').val();
    var local = formatLocal(country, phonenumber);
    var e164 = formatE164(country, phonenumber);
    var sms_on = $('input[type=&quot;radio&quot;][name=&quot;sms_on&quot;]:checked');
    
    // Check the user preference for SMS is set to Yes then validate the number
    if (sms_on_flag == &quot;yes&quot;) {
        phone.parent().find('strong').text(&quot;Telephone Number *&quot;);
        $('#country_select').parent().find('strong').text(&quot;Country *&quot;);
        // country should be selected in order to build the proper phone number
        if (country == &quot;&quot;)
            $('.error_message_country').text('Please select a country.');
        else
            $('.error_message_country').text('');

        // Phone number cannot be empty
        if (phonenumber == &quot;&quot;) {
            $('.error_message_phone_number').text('Please enter phone number.');
        }
        else if (!(isValidNumber(phonenumber, country))) { // phone should be valid 
            $('.error_message_phone_number').text('Invalid Phone Number');
        }
        else {
            $('input[type=&quot;hidden&quot;][name=&quot;phone&quot;]').val(e164.replace('+',''));
                    }
    }
    else if( (sms_on_flag == &quot;no&quot; || sms_on.val() == &quot;no&quot; ) &amp;&amp; phonenumber != '' )
    {
        resetsmson();
        $('#country_select').parent().find('strong').text(&quot;Country *&quot;);
        // country should be selected in order to build the proper phone number
        if (country == &quot;&quot;)
        {
            $('.error_message_country').text('Please select a country.');
            retval=1;
        }
        else
            $('.error_message_country').text('');

        if (!(isValidNumber(phonenumber, country))) { // phone should be valid 
            $('.error_message_phone_number').text('Invalid Phone Number');
            retval=1;
        }
        else {
            $('input[type=&quot;hidden&quot;][name=&quot;phone&quot;]').val(e164.replace('+',''));
                        $('.error_message_phone_number').text('');
        }
    }
    else if(phonenumber == '')
    {
        $('#country_select').parent().find('strong').text(&quot;Country&quot;);
        phone.parent().find('strong').text(&quot;Telephone Number&quot;);
        $('.error_message_phone_number').text('');
        $('.error_message_country').text('');
        $('input[type=&quot;hidden&quot;][name=&quot;phone&quot;]').val('');
    }
}
function check_notification(type, val){
    var otherval = &quot;&quot;;
    // LA-6305 SMS Alert: Implement UI functionality - added sms_on to the array notifications_ids
    var notifications_ids = [&quot;direct_messages&quot;, 
    &quot;comments_on_post&quot;, 
    &quot;comments_on_post_comment&quot;, 
    &quot;events_created_modified&quot;, 
    &quot;notification_required&quot;, 
    &quot;signup_received&quot;, 
    &quot;rsvp_received&quot;, 
    &quot;likes_posts&quot;, 
    &quot;invite_accepts&quot;,
    &quot;sms_on&quot;];
    
    // Check the user preference for SMS is set to Yes then validate the number
    if (type == &quot;sms_on&quot; &amp;&amp; val == &quot;yes&quot;)
    {
        var phone = $('input[type=&quot;text&quot;][name=&quot;phone_view&quot;]');
        phone_val = phone.val();
        var phonenumber = phone.val().replace('+','');
        var country = $('#country_select').val();
        var local = formatLocal(country, phonenumber);
        var e164 = formatE164(country, phonenumber);
        var sms_on = $('input[type=&quot;radio&quot;][name=&quot;sms_on&quot;]:checked');

        var new_phone1 = e164.replace('+','');
        old_phone_number = old_phone_number.replace('+','');
        // alert(old_phone_number);
        // alert(new_phone1);
        // alert(sms_on_flag)
        // alert(old_phone_number != new_phone1);
        
        phone.parent().find('strong').text(&quot;Telephone Number *&quot;);
        $('#country_select').parent().find('strong').text(&quot;Country *&quot;);
        // country should be selected in order to build the proper phone number
        if (country == &quot;&quot;) {
            $('input[type=&quot;radio&quot;][name=&quot;sms_on&quot;][value=&quot;no&quot;]').attr('checked', true);
            $('.error_message_country').text('Please select a country.');
            elgg.warning_message('Please select a country.');
        }
        else {
            $('input[type=&quot;radio&quot;][name=&quot;sms_on&quot;][value=&quot;no&quot;]').attr('checked', true);
            $('.error_message_country').text('');
        }
        
        // Phone number cannot be empty
        if (phonenumber == &quot;&quot;) {
            $('input[type=&quot;radio&quot;][name=&quot;sms_on&quot;][value=&quot;no&quot;]').attr('checked', true);
            $('.error_message_phone_number').text('Please enter phone number.');
            elgg.warning_message('A valid phone number is required to enable text message delivery to your phone.');
            invalid_phone=true;
        }
        else if (!(isValidNumber(phonenumber, country))) { // phone number should be valid
            $('input[type=&quot;radio&quot;][name=&quot;sms_on&quot;][value=&quot;no&quot;]').attr('checked', true);
            $('.error_message_phone_number').text('Invalid Phone Number');
            elgg.warning_message('A valid phone number is required to enable text message delivery to your phone.');
            invalid_phone=true;
        }
        else if(sms_on_flag == 'no' || old_phone_number != new_phone1){
            resetvalidate_phone_number_popup();
            invalid_phone = false;
            $('#validatephonenumModal').modal('show');
            $('input[type=&quot;hidden&quot;][name=&quot;phone&quot;]').val(e164.replace('+',''));
                        $('.error_message_phone_number').text('');
        }
        else {
            resetsmson();
        }
        
        if(old_phone_number != new_phone1 &amp;&amp; sms_on_flag == &quot;yes&quot;) {
            if(invalid_phone==false)
                $('.error_message_phone_number').text('Phone number has to be re-validated to enable the SMS alerts.');
        }
    }
    else
    {
        var phone = $('input[type=&quot;text&quot;][name=&quot;phone_view&quot;]');
        $('#country_select').parent().find('strong').text(&quot;Country&quot;);
        phone.parent().find('strong').text(&quot;Telephone Number&quot;);
        $('.error_message_phone_number').text('');
    }
    // Check the user preference for Autodial is set to Yes then validate the number
    //Added for LA-9977
    if (type == &quot;autodial_on&quot; &amp;&amp; val == &quot;yes&quot;) {
        var phone = $('input[type=&quot;text&quot;][name=&quot;phone_view&quot;]');
        phone_val = phone.val();
        var phonenumber = phone.val().replace('+','');
        // Phone number cannot be empty
        if (phonenumber == &quot;&quot;) {
            $('input[type=&quot;radio&quot;][name=&quot;autodial_on&quot;][value=&quot;no&quot;]').attr('checked', true);
            $('.error_message_phone_number').text('Please enter phone number.');
            elgg.warning_message('A valid phone number is required to enable call to your phone.');
            invalid_phone=true;
        }      
    }
    
    if (val == 'no') {
        if (type == 'email_notification') {
            var selected = $('input[type=&quot;radio&quot;][name=&quot;mobile_push_alert&quot;]:checked');
            if (selected.length > 0) {
                otherval = selected.val();
            }
        } else {
            var selected = $('input[type=&quot;radio&quot;][name=&quot;email_notification&quot;]:checked');
            if (selected.length > 0) {
                otherval = selected.val();
            }
        }
    }

    if (otherval != &quot;&quot; &amp;&amp; otherval == val &amp;&amp; type != 'sms_on') {
        for (var i = 0; i &lt; notifications_ids.length; i++) {
            $('input[type=&quot;checkbox&quot;][name=&quot;'+notifications_ids[i]+'&quot;]').attr('disabled', true);
        }
    } else {
        for (var i = 0; i &lt; notifications_ids.length; i++) {
            $('input[type=&quot;checkbox&quot;][name=&quot;'+notifications_ids[i]+'&quot;]').removeAttr(&quot;disabled&quot;);
        }
    }
}

function changeState() {
    var country = $('#country_select').val();
    var sms_on = $('input[type=&quot;radio&quot;][name=&quot;sms_on&quot;]:checked');

    var phone = $('input[type=&quot;text&quot;][name=&quot;phone_view&quot;]');
    var phonenumber=phone.val();

    if (sms_on.val() == &quot;yes&quot;) {
        if (country == &quot;&quot;)
            $('.error_message_country').text('Please select a country.');
        else
            $('.error_message_country').text('');
    }
    
    elgg.action('get_states_of_country', {
        data : {
            country_code: $('#country_select').val(), 
            selected_state_name: $('#state_selected_value').val()
        },
        success : function(data) {
            var response;
            if ((data) &amp;&amp; data != undefined)
                response = data.output;

                if ((response) &amp;&amp; response != undefined) {                  
                    var statesList = response.states_list;
                    $(&quot;#state_select&quot;).html(statesList);
                }
        }
    });

    //validating phone number against the country chosen 
    $('.error_message_phone_number').html(&quot;&quot;);
    if(chkPhoneOnUpdate(phonenumber,country))
    {

      $(&quot;#lt_page_loader&quot;).hide();
      $('.error_message_phone_number').html(&quot;Invalid phone number or country code.&quot;);
      $('#phone_view').focus();                     
    }
}

var old_form_data;
function getTimeZone() {
    elgg.action('event_manager/timezone/get_timezones_list', {
        data : {

        },
        success : function(data) {
            var response;
            if((data) &amp;&amp; data != undefined)
                response = data.output;

                if((response) &amp;&amp; response != undefined){
                    var tzInfoUser = response.timezones_info.user_tz_info;
                    $('#current_timezone_name').html( tzInfoUser.name_ui);
                    $('#curr_tz_time').html( &quot;- &quot;+ tzInfoUser.curr_time );                  
                    $('#set_timezone #tz_setting_name').html(tzInfoUser.name_ui);

                    var timezonesList = response.timezones_info.tz_list;

                    var timezoneOptionsHtml = &quot;&quot;;

                    $.each(timezonesList, function(regionId, timezones){
                        timezoneOptionsHtml += '&lt;optgroup label=&quot;' + regionId + '&quot;>';
                        $.each(timezones, function(key, tzInfo){
                            var timezoneNameId = tzInfo.name;
                            var timezoneNameUi = tzInfo.name_ui;
                            var timezonecurrentTime = tzInfo.curr_time;

                            timezoneOptionsHtml += '&lt;option class=&quot;timezone_sel_option&quot; value=&quot;' + timezoneNameId + '&quot;>' + timezoneNameUi + &quot; - &quot; + timezonecurrentTime + '&lt;/option>';
                        });
                        timezoneOptionsHtml += '&lt;optgroup/>';

                    });

                    $('#timezone_select_list').html(timezoneOptionsHtml);              
                    $('#timezone_select_list option[value=&quot;'+tzInfoUser.name+'&quot;]').attr(&quot;selected&quot;,true);
                    $('#timezone_select_list').trigger(&quot;liszt:updated&quot;);

                    $('#tz_settings_area_loading').hide();
                    $('#tz_settings_area').show();
                    old_form_data = $('#editprofilefrm').serialize();
                }
        }
    });
}


Edit Profile &amp; Preferences















First Name *






Last Name *






Email





Street Address






City





Zip Code






Country

	Country *
                                    United States
                                        India
                                        Afghanistan
                                        Albania
                                        Algeria
                                        American Samoa
                                        Andorra
                                        Angola
                                        Anguilla
                                        Antarctica
                                        Antigua And Barbuda
                                        Argentina
                                        Armenia
                                        Aruba
                                        Australia
                                        Austria
                                        Azerbaijan
                                        Bahamas, The
                                        Bahrain
                                        Bangladesh
                                        Barbados
                                        Belarus
                                        Belgium
                                        Belize
                                        Benin
                                        Bermuda
                                        Bhutan
                                        Bolivia
                                        Bosnia And Herzegovina
                                        Botswana
                                        Bouvet Island
                                        Brazil
                                        British Indian Ocean Territory
                                        Brunei
                                        Bulgaria
                                        Burkina Faso
                                        Burma
                                        Burundi
                                        Cambodia
                                        Cameroon
                                        Canada
                                        Cape Verde
                                        Cayman Islands
                                        Central African Republic
                                        Chad
                                        Chile
                                        China
                                        Christmas Island
                                        Cocos (keeling) Islands
                                        Colombia
                                        Comoros
                                        Congo (brazzaville) 
                                        Congo (kinshasa)
                                        Cook Islands
                                        Costa Rica
                                        CÔte D’ivoire
                                        Croatia
                                        Cuba
                                        CuraÇao
                                        Cyprus
                                        Czech Republic
                                        Denmark
                                        Djibouti
                                        Dominica
                                        Dominican Republic
                                        Ecuador
                                        Egypt
                                        El Salvador
                                        Equatorial Guinea
                                        Eritrea
                                        Estonia
                                        Ethiopia
                                        Falkland Islands (islas Malvinas)
                                        Faroe Islands
                                        Fiji
                                        Finland
                                        France
                                        French Guiana
                                        French Polynesia
                                        French Southern And Antarctic Lands
                                        Gabon
                                        Gambia, The
                                        Georgia
                                        Germany
                                        Ghana
                                        Gibraltar
                                        Greece
                                        Greenland
                                        Grenada
                                        Guadeloupe
                                        Guam
                                        Guatemala
                                        Guernsey
                                        Guinea
                                        Guinea-bissau
                                        Guyana
                                        Haiti
                                        Heard Island And Mcdonald Islands
                                        Honduras
                                        Hong Kong
                                        Hungary
                                        Iceland
                                        Indonesia
                                        Iran
                                        Iraq
                                        Ireland
                                        Isle Of Man
                                        Israel
                                        Italy
                                        Jamaica
                                        Japan
                                        Jersey
                                        Jordan
                                        Kazakhstan
                                        Kenya
                                        Kiribati
                                        Korea, North
                                        Korea, South
                                        Kuwait
                                        Kyrgyzstan
                                        Laos
                                        Latvia
                                        Lebanon
                                        Lesotho
                                        Liberia
                                        Libya
                                        Liechtenstein
                                        Lithuania
                                        Luxembourg
                                        Macau
                                        Macedonia
                                        Madagascar
                                        Malawi
                                        Malaysia
                                        Maldives
                                        Mali
                                        Malta
                                        Marshall Islands
                                        Martinique
                                        Mauritania
                                        Mauritius
                                        Mayotte
                                        Mexico
                                        Micronesia, Federated States Of
                                        Moldova
                                        Monaco
                                        Mongolia
                                        Montenegro
                                        Montserrat
                                        Morocco
                                        Mozambique
                                        Namibia
                                        Nauru
                                        Nepal
                                        Netherlands
                                        New Caledonia
                                        New Zealand
                                        Nicaragua
                                        Niger
                                        Nigeria
                                        Niue
                                        Norfolk Island
                                        Northern Mariana Islands
                                        Norway
                                        Oman
                                        Pakistan
                                        Palau
                                        Panama
                                        Papua New Guinea
                                        Paraguay
                                        Peru
                                        Philippines
                                        Pitcairn Islands
                                        Poland
                                        Portugal
                                        Puerto Rico
                                        Qatar
                                        Reunion
                                        Romania
                                        Russia
                                        Rwanda
                                        Saint Barthelemy
                                        Saint Helena, Ascension, And Tristan Da Cunha
                                        Saint Kitts And Nevis
                                        Saint Lucia
                                        Saint Martin
                                        Saint Pierre And Miquelon
                                        Saint Vincent And The Grenadines
                                        Samoa
                                        San Marino
                                        Sao Tome And Principe
                                        Saudi Arabia
                                        Senegal
                                        Serbia
                                        Seychelles
                                        Sierra Leone
                                        Singapore
                                        Sint Maarten
                                        Slovakia
                                        Slovenia
                                        Solomon Islands
                                        Somalia
                                        South Africa
                                        South Georgia And South Sandwich Islands
                                        South Sudan
                                        Spain
                                        Sri Lanka
                                        Sudan
                                        Suriname
                                        Swaziland
                                        Sweden
                                        Switzerland
                                        Syria
                                        Taiwan
                                        Tajikistan
                                        Tanzania
                                        Thailand
                                        Timor-leste
                                        Togo
                                        Tokelau
                                        Tonga
                                        Trinidad And Tobago
                                        Tunisia
                                        Turkey
                                        Turkmenistan
                                        Turks And Caicos Islands
                                        Tuvalu
                                        Uganda
                                        Ukraine
                                        United Arab Emirates
                                        United Kingdom
                                        Uruguay
                                        Uzbekistan
                                        Vanuatu
                                        Vatican City
                                        Venezuela
                                        Vietnam
                                        Virgin Islands, British
                                        Virgin Islands, United States 
                                        Wallis And Futuna
                                        Western Sahara
                                        Yemen
                                        Zambia
                                        Zimbabwe
                                    
 



State
Select StateAlabamaAlaskaArizonaArkansasCaliforniaColoradoConnecticutDelawareDistrict of ColumbiaFloridaGeorgiaHawaiiIdahoIllinoisIndianaIowaKansasKentuckyLouisianaMaineMarylandMassachusettsMichiganMinnesotaMississippiMissouriMontanaNebraskaNevadaNew HampshireNew JerseyNew MexicoNew YorkNorth CarolinaNorth DakotaOhioOklahomaOregonPennsylvaniaRhode IslandSouth CarolinaSouth DakotaTennesseeTexasUtahVermontVirginiaWashingtonWest VirginiaWisconsinWyoming 


Time Zone *

                getTimeZone();
                
Hawaii/Aleutian - Thu, 1:34 amUS/Alaska - Thu, 2:34 amUS/Central - Thu, 5:34 amUS/Mountain - Thu, 4:34 amUS/Pacific - Thu, 3:34 amUS/Eastern - Thu, 6:34 amUS/Mountain (Arizona) - Thu, 3:34 amHawaii-Aleutian (No DST) - Thu, 12:34 amIndia Standard Time - Thu, 4:04 pmAsia/Kabul - Thu, 3:04 pmEurope/Tirane - Thu, 12:34 pmAfrica/Algiers - Thu, 11:34 amPacific/Pago_Pago - Wed, 11:34 pmEurope/Andorra - Thu, 12:34 pmAfrica/Luanda - Thu, 11:34 amAmerica/Anguilla - Thu, 6:34 amAntarctica/Casey - Thu, 6:34 pmAntarctica/Davis - Thu, 5:34 pmAntarctica/DumontDUrville - Thu, 8:34 pmAntarctica/Mawson - Thu, 3:34 pmAntarctica/McMurdo - Thu, 10:34 pmAntarctica/Palmer - Thu, 7:34 amAntarctica/Rothera - Thu, 7:34 amAntarctica/Syowa - Thu, 1:34 pmAntarctica/Troll - Thu, 12:34 pmAntarctica/Vostok - Thu, 4:34 pmAmerica/Antigua - Thu, 6:34 amAmerica/Argentina/Buenos_Aires - Thu, 7:34 amAmerica/Argentina/Catamarca - Thu, 7:34 amAmerica/Argentina/Cordoba - Thu, 7:34 amAmerica/Argentina/Jujuy - Thu, 7:34 amAmerica/Argentina/La_Rioja - Thu, 7:34 amAmerica/Argentina/Mendoza - Thu, 7:34 amAmerica/Argentina/Rio_Gallegos - Thu, 7:34 amAmerica/Argentina/Salta - Thu, 7:34 amAmerica/Argentina/San_Juan - Thu, 7:34 amAmerica/Argentina/San_Luis - Thu, 7:34 amAmerica/Argentina/Tucuman - Thu, 7:34 amAmerica/Argentina/Ushuaia - Thu, 7:34 amAsia/Yerevan - Thu, 2:34 pmAmerica/Aruba - Thu, 6:34 amAntarctica/Macquarie - Thu, 9:34 pmAustralia/Adelaide - Thu, 8:04 pmAustralia/Brisbane - Thu, 8:34 pmAustralia/Broken_Hill - Thu, 8:04 pmAustralia/Currie - Thu, 8:34 pmAustralia/Darwin - Thu, 8:04 pmAustralia/Eucla - Thu, 7:19 pmAustralia/Hobart - Thu, 8:34 pmAustralia/Lindeman - Thu, 8:34 pmAustralia/Lord_Howe - Thu, 9:04 pmAustralia/Melbourne - Thu, 8:34 pmAustralia/Perth - Thu, 6:34 pmAustralia/Sydney - Thu, 8:34 pmEurope/Vienna - Thu, 12:34 pmAsia/Baku - Thu, 2:34 pmAmerica/Nassau - Thu, 6:34 amAsia/Bahrain - Thu, 1:34 pmAsia/Dhaka - Thu, 4:34 pmAmerica/Barbados - Thu, 6:34 amEurope/Minsk - Thu, 1:34 pmEurope/Brussels - Thu, 12:34 pmAmerica/Belize - Thu, 4:34 amAfrica/Porto-Novo - Thu, 11:34 amAtlantic/Bermuda - Thu, 7:34 amAsia/Thimphu - Thu, 4:34 pmAmerica/La_Paz - Thu, 6:34 amEurope/Sarajevo - Thu, 12:34 pmAfrica/Gaborone - Thu, 12:34 pmAmerica/Araguaina - Thu, 7:34 amAmerica/Bahia - Thu, 7:34 amAmerica/Belem - Thu, 7:34 amAmerica/Boa_Vista - Thu, 6:34 amAmerica/Campo_Grande - Thu, 6:34 amAmerica/Cuiaba - Thu, 6:34 amAmerica/Eirunepe - Thu, 5:34 amAmerica/Fortaleza - Thu, 7:34 amAmerica/Maceio - Thu, 7:34 amAmerica/Manaus - Thu, 6:34 amAmerica/Noronha - Thu, 8:34 amAmerica/Porto_Velho - Thu, 6:34 amAmerica/Recife - Thu, 7:34 amAmerica/Rio_Branco - Thu, 5:34 amAmerica/Santarem - Thu, 7:34 amAmerica/Sao_Paulo - Thu, 7:34 amIndian/Chagos - Thu, 4:34 pmAsia/Brunei - Thu, 6:34 pmEurope/Sofia - Thu, 1:34 pmAfrica/Ouagadougou - Thu, 10:34 amAsia/Yangon - Thu, 5:04 pmAfrica/Bujumbura - Thu, 12:34 pmAsia/Phnom_Penh - Thu, 5:34 pmAfrica/Douala - Thu, 11:34 amAmerica/Atikokan - Thu, 5:34 amAmerica/Blanc-Sablon - Thu, 6:34 amAmerica/Cambridge_Bay - Thu, 4:34 amAmerica/Creston - Thu, 3:34 amAmerica/Dawson - Thu, 3:34 amAmerica/Dawson_Creek - Thu, 3:34 amAmerica/Edmonton - Thu, 4:34 amAmerica/Fort_Nelson - Thu, 3:34 amAmerica/Glace_Bay - Thu, 7:34 amAmerica/Goose_Bay - Thu, 7:34 amAmerica/Halifax - Thu, 7:34 amAmerica/Inuvik - Thu, 4:34 amAmerica/Iqaluit - Thu, 6:34 amAmerica/Moncton - Thu, 7:34 amAmerica/Nipigon - Thu, 6:34 amAmerica/Pangnirtung - Thu, 6:34 amAmerica/Rainy_River - Thu, 5:34 amAmerica/Rankin_Inlet - Thu, 5:34 amAmerica/Regina - Thu, 4:34 amAmerica/Resolute - Thu, 5:34 amAmerica/St_Johns - Thu, 8:04 amAmerica/Swift_Current - Thu, 4:34 amAmerica/Thunder_Bay - Thu, 6:34 amAmerica/Toronto - Thu, 6:34 amAmerica/Vancouver - Thu, 3:34 amAmerica/Whitehorse - Thu, 3:34 amAmerica/Winnipeg - Thu, 5:34 amAmerica/Yellowknife - Thu, 4:34 amAtlantic/Cape_Verde - Thu, 9:34 amAmerica/Cayman - Thu, 5:34 amAfrica/Bangui - Thu, 11:34 amAfrica/Ndjamena - Thu, 11:34 amAmerica/Punta_Arenas - Thu, 7:34 amAmerica/Santiago - Thu, 6:34 amPacific/Easter - Thu, 4:34 amAsia/Shanghai - Thu, 6:34 pmAsia/Urumqi - Thu, 4:34 pmIndian/Christmas - Thu, 5:34 pmIndian/Cocos - Thu, 5:04 pmAmerica/Bogota - Thu, 5:34 amIndian/Comoro - Thu, 1:34 pmAfrica/Brazzaville - Thu, 11:34 amAfrica/Kinshasa - Thu, 11:34 amAfrica/Lubumbashi - Thu, 12:34 pmPacific/Rarotonga - Thu, 12:34 amAmerica/Costa_Rica - Thu, 4:34 amAfrica/Abidjan - Thu, 10:34 amEurope/Zagreb - Thu, 12:34 pmAmerica/Havana - Thu, 6:34 amAmerica/Curacao - Thu, 6:34 amAsia/Famagusta - Thu, 1:34 pmAsia/Nicosia - Thu, 1:34 pmEurope/Prague - Thu, 12:34 pmEurope/Copenhagen - Thu, 12:34 pmAfrica/Djibouti - Thu, 1:34 pmAmerica/Dominica - Thu, 6:34 amAmerica/Santo_Domingo - Thu, 6:34 amAmerica/Guayaquil - Thu, 5:34 amPacific/Galapagos - Thu, 4:34 amAfrica/Cairo - Thu, 12:34 pmAmerica/El_Salvador - Thu, 4:34 amAfrica/Malabo - Thu, 11:34 amAfrica/Asmara - Thu, 1:34 pmEurope/Tallinn - Thu, 1:34 pmAfrica/Addis_Ababa - Thu, 1:34 pmAtlantic/Stanley - Thu, 7:34 amAtlantic/Faroe - Thu, 11:34 amPacific/Fiji - Thu, 10:34 pmEurope/Helsinki - Thu, 1:34 pmEurope/Paris - Thu, 12:34 pmAmerica/Cayenne - Thu, 7:34 amPacific/Gambier - Thu, 1:34 amPacific/Marquesas - Thu, 1:04 amPacific/Tahiti - Thu, 12:34 amIndian/Kerguelen - Thu, 3:34 pmAfrica/Libreville - Thu, 11:34 amAfrica/Banjul - Thu, 10:34 amAsia/Tbilisi - Thu, 2:34 pmEurope/Berlin - Thu, 12:34 pmEurope/Busingen - Thu, 12:34 pmAfrica/Accra - Thu, 10:34 amEurope/Gibraltar - Thu, 12:34 pmEurope/Athens - Thu, 1:34 pmAmerica/Danmarkshavn - Thu, 10:34 amAmerica/Godthab - Thu, 8:34 amAmerica/Scoresbysund - Thu, 10:34 amAmerica/Thule - Thu, 7:34 amAmerica/Grenada - Thu, 6:34 amAmerica/Guadeloupe - Thu, 6:34 amPacific/Guam - Thu, 8:34 pmAmerica/Guatemala - Thu, 4:34 amEurope/Guernsey - Thu, 11:34 amAfrica/Conakry - Thu, 10:34 amAfrica/Bissau - Thu, 10:34 amAmerica/Guyana - Thu, 6:34 amAmerica/Port-au-Prince - Thu, 6:34 amAmerica/Tegucigalpa - Thu, 4:34 amAsia/Hong_Kong - Thu, 6:34 pmEurope/Budapest - Thu, 12:34 pmAtlantic/Reykjavik - Thu, 10:34 amAsia/Jakarta - Thu, 5:34 pmAsia/Jayapura - Thu, 7:34 pmAsia/Makassar - Thu, 6:34 pmAsia/Pontianak - Thu, 5:34 pmAsia/Tehran - Thu, 3:04 pmAsia/Baghdad - Thu, 1:34 pmEurope/Dublin - Thu, 11:34 amEurope/Isle_of_Man - Thu, 11:34 amAsia/Jerusalem - Thu, 1:34 pmEurope/Rome - Thu, 12:34 pmAmerica/Jamaica - Thu, 5:34 amAsia/Tokyo - Thu, 7:34 pmEurope/Jersey - Thu, 11:34 amAsia/Amman - Thu, 1:34 pmAsia/Almaty - Thu, 4:34 pmAsia/Aqtau - Thu, 3:34 pmAsia/Aqtobe - Thu, 3:34 pmAsia/Atyrau - Thu, 3:34 pmAsia/Oral - Thu, 3:34 pmAsia/Qostanay - Thu, 4:34 pmAsia/Qyzylorda - Thu, 3:34 pmAfrica/Nairobi - Thu, 1:34 pmPacific/Enderbury - Thu, 11:34 pmPacific/Kiritimati - Fri, 12:34 amPacific/Tarawa - Thu, 10:34 pmAsia/Pyongyang - Thu, 7:34 pmAsia/Seoul - Thu, 7:34 pmAsia/Kuwait - Thu, 1:34 pmAsia/Bishkek - Thu, 4:34 pmAsia/Vientiane - Thu, 5:34 pmEurope/Riga - Thu, 1:34 pmAsia/Beirut - Thu, 1:34 pmAfrica/Maseru - Thu, 12:34 pmAfrica/Monrovia - Thu, 10:34 amAfrica/Tripoli - Thu, 12:34 pmEurope/Vaduz - Thu, 12:34 pmEurope/Vilnius - Thu, 1:34 pmEurope/Luxembourg - Thu, 12:34 pmAsia/Macau - Thu, 6:34 pmEurope/Skopje - Thu, 12:34 pmIndian/Antananarivo - Thu, 1:34 pmAfrica/Blantyre - Thu, 12:34 pmAsia/Kuala_Lumpur - Thu, 6:34 pmAsia/Kuching - Thu, 6:34 pmIndian/Maldives - Thu, 3:34 pmAfrica/Bamako - Thu, 10:34 amEurope/Malta - Thu, 12:34 pmPacific/Kwajalein - Thu, 10:34 pmPacific/Majuro - Thu, 10:34 pmAmerica/Martinique - Thu, 6:34 amAfrica/Nouakchott - Thu, 10:34 amIndian/Mauritius - Thu, 2:34 pmIndian/Mayotte - Thu, 1:34 pmAmerica/Bahia_Banderas - Thu, 5:34 amAmerica/Cancun - Thu, 5:34 amAmerica/Chihuahua - Thu, 4:34 amAmerica/Hermosillo - Thu, 3:34 amAmerica/Matamoros - Thu, 5:34 amAmerica/Mazatlan - Thu, 4:34 amAmerica/Merida - Thu, 5:34 amAmerica/Mexico_City - Thu, 5:34 amAmerica/Monterrey - Thu, 5:34 amAmerica/Ojinaga - Thu, 4:34 amAmerica/Tijuana - Thu, 3:34 amPacific/Chuuk - Thu, 8:34 pmPacific/Kosrae - Thu, 9:34 pmPacific/Pohnpei - Thu, 9:34 pmEurope/Chisinau - Thu, 1:34 pmEurope/Monaco - Thu, 12:34 pmAsia/Choibalsan - Thu, 6:34 pmAsia/Hovd - Thu, 5:34 pmAsia/Ulaanbaatar - Thu, 6:34 pmEurope/Podgorica - Thu, 12:34 pmAmerica/Montserrat - Thu, 6:34 amAfrica/Casablanca - Thu, 11:34 amAfrica/Maputo - Thu, 12:34 pmAfrica/Windhoek - Thu, 12:34 pmPacific/Nauru - Thu, 10:34 pmAsia/Kathmandu - Thu, 4:19 pmEurope/Amsterdam - Thu, 12:34 pmPacific/Noumea - Thu, 9:34 pmPacific/Auckland - Thu, 10:34 pmPacific/Chatham - Thu, 11:19 pmAmerica/Managua - Thu, 4:34 amAfrica/Niamey - Thu, 11:34 amAfrica/Lagos - Thu, 11:34 amPacific/Niue - Wed, 11:34 pmPacific/Norfolk - Thu, 9:34 pmPacific/Saipan - Thu, 8:34 pmEurope/Oslo - Thu, 12:34 pmAsia/Muscat - Thu, 2:34 pmAsia/Karachi - Thu, 3:34 pmPacific/Palau - Thu, 7:34 pmAmerica/Panama - Thu, 5:34 amPacific/Bougainville - Thu, 9:34 pmPacific/Port_Moresby - Thu, 8:34 pmAmerica/Asuncion - Thu, 6:34 amAmerica/Lima - Thu, 5:34 amAsia/Manila - Thu, 6:34 pmPacific/Pitcairn - Thu, 2:34 amEurope/Warsaw - Thu, 12:34 pmAtlantic/Azores - Thu, 10:34 amAtlantic/Madeira - Thu, 11:34 amEurope/Lisbon - Thu, 11:34 amAmerica/Puerto_Rico - Thu, 6:34 amAsia/Qatar - Thu, 1:34 pmIndian/Reunion - Thu, 2:34 pmEurope/Bucharest - Thu, 1:34 pmAsia/Anadyr - Thu, 10:34 pmAsia/Barnaul - Thu, 5:34 pmAsia/Chita - Thu, 7:34 pmAsia/Irkutsk - Thu, 6:34 pmAsia/Kamchatka - Thu, 10:34 pmAsia/Khandyga - Thu, 7:34 pmAsia/Krasnoyarsk - Thu, 5:34 pmAsia/Magadan - Thu, 9:34 pmAsia/Novokuznetsk - Thu, 5:34 pmAsia/Novosibirsk - Thu, 5:34 pmAsia/Omsk - Thu, 4:34 pmAsia/Sakhalin - Thu, 9:34 pmAsia/Srednekolymsk - Thu, 9:34 pmAsia/Tomsk - Thu, 5:34 pmAsia/Ust-Nera - Thu, 8:34 pmAsia/Vladivostok - Thu, 8:34 pmAsia/Yakutsk - Thu, 7:34 pmAsia/Yekaterinburg - Thu, 3:34 pmEurope/Astrakhan - Thu, 2:34 pmEurope/Kaliningrad - Thu, 12:34 pmEurope/Kirov - Thu, 1:34 pmEurope/Moscow - Thu, 1:34 pmEurope/Samara - Thu, 2:34 pmEurope/Saratov - Thu, 2:34 pmEurope/Simferopol - Thu, 1:34 pmEurope/Ulyanovsk - Thu, 2:34 pmEurope/Volgograd - Thu, 2:34 pmAfrica/Kigali - Thu, 12:34 pmAmerica/St_Barthelemy - Thu, 6:34 amAtlantic/St_Helena - Thu, 10:34 amAmerica/St_Kitts - Thu, 6:34 amAmerica/St_Lucia - Thu, 6:34 amAmerica/Marigot - Thu, 6:34 amAmerica/Miquelon - Thu, 8:34 amAmerica/St_Vincent - Thu, 6:34 amPacific/Apia - Thu, 11:34 pmEurope/San_Marino - Thu, 12:34 pmAfrica/Sao_Tome - Thu, 10:34 amAsia/Riyadh - Thu, 1:34 pmAfrica/Dakar - Thu, 10:34 amEurope/Belgrade - Thu, 12:34 pmIndian/Mahe - Thu, 2:34 pmAfrica/Freetown - Thu, 10:34 amAsia/Singapore - Thu, 6:34 pmAmerica/Lower_Princes - Thu, 6:34 amEurope/Bratislava - Thu, 12:34 pmEurope/Ljubljana - Thu, 12:34 pmPacific/Guadalcanal - Thu, 9:34 pmAfrica/Mogadishu - Thu, 1:34 pmAfrica/Johannesburg - Thu, 12:34 pmAtlantic/South_Georgia - Thu, 8:34 amAfrica/Juba - Thu, 1:34 pmAfrica/Ceuta - Thu, 12:34 pmAtlantic/Canary - Thu, 11:34 amEurope/Madrid - Thu, 12:34 pmAsia/Colombo - Thu, 4:04 pmAfrica/Khartoum - Thu, 12:34 pmAmerica/Paramaribo - Thu, 7:34 amAfrica/Mbabane - Thu, 12:34 pmEurope/Stockholm - Thu, 12:34 pmEurope/Zurich - Thu, 12:34 pmAsia/Damascus - Thu, 1:34 pmAsia/Taipei - Thu, 6:34 pmAsia/Dushanbe - Thu, 3:34 pmAfrica/Dar_es_Salaam - Thu, 1:34 pmAsia/Bangkok - Thu, 5:34 pmAsia/Dili - Thu, 7:34 pmAfrica/Lome - Thu, 10:34 amPacific/Fakaofo - Thu, 11:34 pmPacific/Tongatapu - Thu, 11:34 pmAmerica/Port_of_Spain - Thu, 6:34 amAfrica/Tunis - Thu, 11:34 amEurope/Istanbul - Thu, 1:34 pmAsia/Ashgabat - Thu, 3:34 pmAmerica/Grand_Turk - Thu, 6:34 amPacific/Funafuti - Thu, 10:34 pmAfrica/Kampala - Thu, 1:34 pmEurope/Kiev - Thu, 1:34 pmEurope/Uzhgorod - Thu, 1:34 pmEurope/Zaporozhye - Thu, 1:34 pmAsia/Dubai - Thu, 2:34 pmEurope/London - Thu, 11:34 amAmerica/Montevideo - Thu, 7:34 amAsia/Samarkand - Thu, 3:34 pmAsia/Tashkent - Thu, 3:34 pmPacific/Efate - Thu, 9:34 pmEurope/Vatican - Thu, 12:34 pmAmerica/Caracas - Thu, 6:34 amAsia/Ho_Chi_Minh - Thu, 5:34 pmAmerica/Tortola - Thu, 6:34 amAmerica/St_Thomas - Thu, 6:34 amPacific/Wallis - Thu, 10:34 pmAfrica/El_Aaiun - Thu, 11:34 amAsia/Aden - Thu, 1:34 pmAfrica/Lusaka - Thu, 12:34 pmAfrica/Harare - Thu, 12:34 pm




Telephone Number


 



Gender

                Select Gender
                Male
                Female
                


Birthday
$(document).ready(function() { $('#editprofilefrm').find('.birth_mdy').datepicker({
                    dateFormat: 'mm/dd/yy',
                        changeMonth: true,
                        changeYear: true,
                        yearRange: '1900:',
                        maxDate: '+0d',
                        showMonthAfterYear: false,
                        onChangeMonthYear:function(y, m, i){                                
                                var d = i.selectedDay;
                                    $(this).datepicker('setDate', new Date(y, m - 1, d));
                        },
                            onSelect:function() {
                                 $(this).data('datepicker').inline = true;  
                                  },
                                    onClose: function() {
                                        $(this).data('datepicker').inline = false;
                                    },
                                                           
                })
                $(window).bind('beforeunload', function (event) {
                    if(old_form_data != $('#editprofilefrm').serialize()) {
                        return 'Please save information before leaving';
                    }
                    //(event.preventDefault) ? event.preventDefault() : event.returnValue = false;
                });
                });


Language
AfrikaansAlbanianAmharicArabicArmenianAzerbaijaniBasqueBelarusianBengaliBosnianBulgarianCatalanCebuanoChichewaChinese SimplifiedChinese TraditionalCorsicanCroatianCzechDanishDutchEnglishEsperantoEstonianFilipinoFinnishFrenchFrisianGalicianGeorgianGermanGreekGujaratiHaitian CreoleHausaHawaiianHebrewHindiHmongHmong DawHungarianIcelandicIgboIndonesianIrishItalianJapaneseJavaneseKannadaKazakhKhmerKlingonKlingon (pIqaD)KoreanKurdish (Kurmanji)KyrgyzLaoLatinLatvianLithuanianLuxembourgishMacedonianMalagasyMalayMalayalamMalteseMaoriMarathiMongolianMyanmar (Burmese)NepaliNorwegianPashtoPersianPolishPortuguesePunjabiRomanianRussianSamoanScots GaelicSerbianSesothoShonaSindhiSinhalaSlovakSlovenianSomaliSpanishSundaneseSwahiliSwedishTajikTamilTeluguThaiTurkishUkrainianUrduUzbekVietnameseWelshXhosaYiddishYorubaZulu

            Communication Options
            



	Send me a Daily Summary email of LivingTree
		activity
	 Yes  No












	Allow phone message posts/attendance alerts to
		call my phone
	 Yes  No



            Notification Preferences
            



	Email Notifications  Yes  No







            Activities That Will Send a Notification
            




	
	Direct Messages/Attendance Alerts






	
	Comments On My Posts






	
	Comments On Posts That I Have Also Commented






	
	Notification For Events RSVP or Sign-ups






	
	Event Reminder Notifications






	
	Sign-ups Received For Events I Create






	
	RSVPs Received For Events I Create






	
	Likes Received For My Posts






	
	Notifications On Invite Accepts






            Email Reporting (School/District/Org Admins Only)
            



	
	Receive Weekly Sign Up Report






	
	Receive Weekly Activity Report





















    




	
		
			
				
				
					Validate Phone Number
				
			
			
				Your phone number must be validated in order to receive text
					messages from LivingTree. Click the Send Code button to send a text
					message to your phone with an activation code. When you receive the
					activation code, enter it in the Activation Code field and click
					OK.
				 
				Activation
					Code: 
				
					
				
				
					  
					Send Code
				
				
					 
					
				

				
			

			
				OK
				CANCEL
			
		
	



.form-group #edit_profile_checkbox, .form-group #edit_profile_radio {
	height: auto;
	vertical-align: top;
}

.comm_option {
	width: 425px;
	border: 0px solid black;
	float: left;
}

.form-group #edit_profile_text {
	height: auto;
	margin: 0 0 30px 0;
}

#myModalLabel {
	margin-left: 15px;
}


    var sms_on_flag,old_phone_number,invalid_phone;
    $(document).ready(function() {
        var phone = $('input[type=&quot;text&quot;][name=&quot;phone_view&quot;]');
        var sms_on = $('input[type=&quot;radio&quot;][name=&quot;sms_on&quot;]:checked');
        sms_on_flag = sms_on.val();
        old_phone_number = $('input[name=&quot;phone&quot;]').val().replace('+',''); 
        invalid_phone = false;
        if(sms_on.val() == 'yes') {
            phone.parent().find('strong').text(&quot;Telephone Number *&quot;);
            $('#country_select').parent().find('strong').text(&quot;Country *&quot;);
        }
        var autodial_on = $('input[name=&quot;autodialpref&quot;]').val();        
        if(autodial_on == &quot;&quot;) {
            $('input[type=&quot;radio&quot;][name=&quot;autodial_on&quot;][value=&quot;yes&quot;]').attr('checked', true);
        } 
        
        $(&quot;#street_privacy&quot;).click(function() {
            if ( $(&quot;#street_privacy&quot;).is(&quot;:checked&quot;) ) {
                this.value = &quot;yes&quot;;
            } else
            this.value = &quot;no&quot;;
        });
        $(&quot;#phone_number_privacy&quot;).click(function() {
            if ( $(&quot;#phone_number_privacy&quot;).is(&quot;:checked&quot;) ) {
                this.value = &quot;yes&quot;;
            } else
            this.value = &quot;no&quot;;
        });
        $(&quot;#email_privacy&quot;).click(function() {
            if ( $(&quot;#email_privacy&quot;).is(&quot;:checked&quot;) ) {
                this.value = &quot;yes&quot;;
            } else
            this.value = &quot;no&quot;;
        });
        _gaq.push(['_trackEvent', 'LTWeb', 'Edit Profile Preferences']);

    });
    
</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;editprofilefrm&quot;)/fieldset[1]</value>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:idRelative</name>
      <type>Main</type>
      <value>//form[@id='editprofilefrm']/fieldset</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Manage Credit Card'])[1]/following::fieldset[1]</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Edit Profile &amp; Preferences'])[1]/following::fieldset[1]</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//fieldset</value>
   </webElementXpaths>
</WebElementEntity>
