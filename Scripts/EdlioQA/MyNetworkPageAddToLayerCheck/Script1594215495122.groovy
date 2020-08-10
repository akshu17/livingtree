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

WebUI.setText(findTestObject('LivingTree/LoginPage/LoginPageEmailAdressTextBox'), GlobalVariable.EMAIL_PRINCIPLE)

WebUI.setText(findTestObject('LivingTree/LoginPage/LoginPagePasswordTextBox'), GlobalVariable.PASSWORD)

WebUI.click(findTestObject('LivingTree/LoginPage/LoginPageLoginButton'))

WebUI.click(findTestObject('LivingTree/MyNetworkPage/MyNetworkPageIcon'))

WebUI.click(findTestObject('Edlio/CreateNewLayer/BasavaSchool'))

WebUI.click(findTestObject('LivingTree/CreateNewLayer/CreateNewLayerBox'))

WebUI.mouseOver(findTestObject('LivingTree/CreateNewLayer/CreateNewLayerInputBox'))

WebUI.setText(findTestObject('LivingTree/CreateNewLayer/CreateNewLayerInputBox'), 'test')

WebUI.delay(3)

WebUI.verifyElementClickable(findTestObject('Edlio/CreateNewLayer/CreateNewLayerSaveButton'))

WebUI.click(findTestObject('Edlio/CreateNewLayer/CreateNewLayerSaveButton'))

WebUI.delay(2)

WebUI.waitForElementVisible(findTestObject('LivingTree/CreateNewLayer/CreateNewLayerAddOthersButtonCheck'), 1)

WebUI.closeBrowser()

