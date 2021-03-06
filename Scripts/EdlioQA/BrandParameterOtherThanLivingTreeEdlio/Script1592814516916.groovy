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

WebUI.openBrowser('https://maple.livingtree.com/login?brand=edlioedfghj')

WebUI.setText(findTestObject('LivingTree/LoginPage/LoginPageEmailAdressTextBox'), 'teacherbasava@mailinator.com')

WebUI.setText(findTestObject('LivingTree/LoginPage/LoginPagePasswordTextBox'), 'E@tM0r3CH1CK3n')

WebUI.click(findTestObject('LivingTree/LoginPage/LoginPageLoginButton'))

WebUI.verifyElementPresent(findTestObject('LivingTree/ConversationPage/AllIcon'), 0)

WebUI.closeBrowser()

