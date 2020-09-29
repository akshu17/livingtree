<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>ManageCreditCard</name>
   <tag></tag>
   <elementGuidId>c3b448d2-6407-4fae-878a-663c4bc1d64f</elementGuidId>
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
		Manage Credit Card          
                            Cardholder Name
                           
                       
                       
                           Card Number
                           
                       
                       
                            CVC
                           
                      
                      
                            Expiration
                            
                       
                       
                           ZIP/Postal code
                           
                       SAVECANCELvar stripe = Stripe('pk_test_yNWctCWSMSDQCapPegh3KyHQ');
        var styleCC = {
                  base: {
                    color: '#32325d',
                    lineHeight: '24px',
                    fontFamily: 'Helvetica Neue, Helvetica, sans-serif',
                    fontSmoothing: 'antialiased',
                    fontSize: '16px',
                    '::placeholder': {
                      color: '#aab7c4'
                    }
                  },
                  invalid: {
                    color: '#fa755a',
                    iconColor: '#fa755a'
                  }
                };
            $(document).ready(function() {
             initCCElements();
              $('#update-card').live('click', function(event)
              {
               $('#divSavedCard').hide();
                $('#ccForm').show();
              });
              $('#delete-card').live('click', function(event)
              {
                if(confirm('Are you sure you want to delete this card?'))
                    {
                     removeCard();
                    }
              });
               $('.btn-card-cancel').live('click', function(event)
               {
                location.reload();
               });
                $('#editcardfrm').submit(function(e){
                  e.preventDefault();
                   saveCard();
                });
            });
            var cardNumber;
            function initCCElements()
            {
                var elements = stripe.elements();
                cardNumber = elements.create('cardNumber', {style: styleCC});
                cardNumber.mount('#card-number');
                var cardExpiry = elements.create('cardExpiry', {style: styleCC});
                cardExpiry.mount('#card-expiry');
                var cardCvc = elements.create('cardCvc', {style: styleCC});
                cardCvc.mount('#card-cvc');
                var cardPostalCode = elements.create('postalCode', {style: styleCC,value:''});
                cardPostalCode.mount('#card-postal-code');
             }
             function saveCard()
             {
                $('#card-errors').html('');
                if($('#cardholder-name').val() == '')
                {
                    $('#card-errors').html('Please enter cardholder name.');
                    return;
                }
                 $('#card-errors').html('&lt;font style=color:#222;;>Saving...&lt;/font>');
                stripe.createToken(cardNumber,{name:$('#cardholder-name').val()} ).then(function(result) {
                        if (result.error) {
                          $('#card-errors').html(result.error.message);
                          return;
                        } else {
                          saveCardToken(result.token.id);
                        }
                 });
             }
            function saveCardToken(token)
            {
                $('#editcardfrm').ajaxSubmit(
                {
                    url: form.action, type: 'post',data: {'token':token,'mode':'modify'},
                    success:  function(response) {
                        r=JSON.parse(response);
                        if(r.response=='success')
                        {
                           $('#card-errors').html('');
                           elgg.flash_message('Card data saved successfully.',5000);
                           location.reload();
                        }
                        else  if(r.response=='error')
                        {
                            $('#card-errors').html('Saving failed. Please try again.');
                        }
                    }
                });
            }
            function removeCard()
            {
                $('#card-modify-message').html('&lt;font style=color:#222;;>Deleting...&lt;/font>');
                $('#editcardfrm').ajaxSubmit(
                {
                    url: form.action, type: 'post',data: {'mode':'remove'},
                    success:  function(response) {
                        r=JSON.parse(response);
                        if(r.response=='success')
                        {
                           $('#card-modify-message').html('');
                           elgg.flash_message('Card removed successfully.',5000);
                           location.reload();
                        }
                        else  if(r.response=='error')
                        {
                            $('#card-modify-message').html('Failed removing card. Please try again.');
                        }
                    }
                });
            }
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
