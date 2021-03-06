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

WebUI.setText(findTestObject('LivingTree/ConversationPage/ConversationPageInputSearch'), 'test')

WebUI.mouseOver(findTestObject('LivingTree/SearchBox/SearchIcon'))

WebUI.click(findTestObject('LivingTree/SearchBox/SearchIcon'))

WebUI.delay(3)

WebUI.mouseOver(findTestObject('LivingTree/ConversationPage/ConversationSearchBox'))

WebUI.setText(findTestObject('LivingTree/ConversationPage/ConversationPageInputSearch'), 'Photo post')

WebUI.mouseOver(findTestObject('LivingTree/SearchBox/SearchIcon'))

WebUI.click(findTestObject('LivingTree/SearchBox/SearchIcon'))

WebUI.delay(3)

WebUI.mouseOver(findTestObject('LivingTree/ConversationPage/ConversationSearchBox'))

WebUI.setText(findTestObject('LivingTree/ConversationPage/ConversationPageInputSearch'), 'Video post')

WebUI.mouseOver(findTestObject('LivingTree/SearchBox/SearchIcon'))

WebUI.click(findTestObject('LivingTree/SearchBox/SearchIcon'))

WebUI.delay(3)

WebUI.mouseOver(findTestObject('LivingTree/ConversationPage/ConversationSearchBox'))

WebUI.setText(findTestObject('LivingTree/ConversationPage/ConversationPageInputSearch'), 'UI event')

WebUI.mouseOver(findTestObject('LivingTree/SearchBox/SearchIcon'))

WebUI.click(findTestObject('LivingTree/SearchBox/SearchIcon'))

WebUI.delay(3)

WebUI.mouseOver(findTestObject('LivingTree/ConversationPage/ConversationSearchBox'))

WebUI.setText(findTestObject('LivingTree/ConversationPage/ConversationPageInputSearch'), 'file post')

WebUI.mouseOver(findTestObject('LivingTree/SearchBox/SearchIcon'))

WebUI.click(findTestObject('LivingTree/SearchBox/SearchIcon'))

WebUI.delay(3)

WebUI.setText(findTestObject('LivingTree/ConversationPage/ConversationPageInputSearch'), 'poll post')

WebUI.mouseOver(findTestObject('LivingTree/SearchBox/SearchIcon'))

WebUI.click(findTestObject('LivingTree/SearchBox/SearchIcon'))

WebUI.delay(3)

WebUI.closeBrowser()

