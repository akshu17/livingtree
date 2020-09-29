import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import internal.GlobalVariable as GlobalVariable

WebUI.openBrowser(GlobalVariable.URL)

WebUI.setText(findTestObject('LivingTree/LoginPage/LoginPageEmailAdressTextBox'), GlobalVariable.EMAIL_TEACHER)

WebUI.setText(findTestObject('LivingTree/LoginPage/LoginPagePasswordTextBox'), GlobalVariable.PASSWORD)

WebUI.click(findTestObject('LivingTree/LoginPage/LoginPageLoginButton'))

WebUI.click(findTestObject('LivingTree/ConversationPage/ConversationIconCoversationPage'))

WebUI.delay(3)

WebUI.mouseOver(findTestObject('LivingTree/ConversationPage/ConversationSearchBox'))

WebUI.setText(findTestObject('LivingTree/ConversationPage/ConversationPageInputSearch'), 'UI event')

WebUI.click(findTestObject('LivingTree/SearchBox/SearchIcon'))

WebUI.waitForElementPresent(findTestObject('LivingTree/ConversationPage/ConversationUiEventTitle'), 2)

WebUI.click(findTestObject('LivingTree/Calendar/RSVPRequiredButton'))

WebUI.waitForElementVisible(findTestObject('LivingTree/Calendar/UIEventPostBox'), 10)

WebUI.waitForElementVisible(findTestObject('LivingTree/Calendar/UIEvenTitle'), 2)

WebUI.waitForElementVisible(findTestObject('LivingTree/Calendar/RSVPRequiredSignUpCheck'), 2)

WebUI.waitForElementVisible(findTestObject('LivingTree/Calendar/UIEventDetailPage'), 0)

WebUI.closeBrowser()

WebUI.openBrowser(GlobalVariable.URL)

WebUI.setText(findTestObject('LivingTree/LoginPage/LoginPageEmailAdressTextBox'), GlobalVariable.EMAIL_STUDENT)

WebUI.setText(findTestObject('LivingTree/LoginPage/LoginPagePasswordTextBox'), GlobalVariable.PASSWORD)

WebUI.click(findTestObject('LivingTree/LoginPage/LoginPageLoginButton'))

WebUI.delay(2)

WebUI.mouseOver(findTestObject('LivingTree/Calendar/ConversationPageShareBox'))

WebUI.setText(findTestObject('LivingTree/SearchBox/SearchBox'), 'UI event')

WebUI.click(findTestObject('LivingTree/SearchBox/SearchIcon'))

WebUI.waitForElementVisible(findTestObject('LivingTree/Calendar/StudentUIEventCheck'), 1)

WebUI.waitForElementPresent(findTestObject('LivingTree/Calendar/SeeOriginal'), 0)

WebUI.delay(3)

WebUI.click(findTestObject('LivingTree/Calendar/a_RSVP Required'))

WebUI.waitForElementVisible(findTestObject('LivingTree/Calendar/UIEventDetailPage'), 0)

WebUI.closeBrowser()

